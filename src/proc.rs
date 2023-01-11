use core::mem::MaybeUninit;
use spin::{Mutex, RwLock};

use crate::mem_utils::{slice_cpy};
use crate::memolayout::{get_trampoline, get_uservec, TRAMPOLINE, TRAPFRAME};
use crate::params::{NCPU, NPROC};
use crate::riscv::{r_tp, w_stvec, PGSIZE, PTE_R, PTE_W, PTE_X};
use crate::vm::{kalloc, mappages, uvmcreate, uvminit, PageTable};
// Saved registers for kernel context switches.

pub static mut next_pid: Mutex<i32> = Mutex::new(1);

pub static mut proc: [RwLock<Proc>; NPROC] = unsafe { MaybeUninit::zeroed().assume_init() }; // because this is convient
pub static mut cpus: [RwLock<Cpu>; NCPU] = unsafe { MaybeUninit::zeroed().assume_init() };

pub static initcode: [u8; 52] = [
    0x17, 0x05, 0x00, 0x00, 0x13, 0x05, 0x45, 0x02, 0x97, 0x05, 0x00, 0x00, 0x93, 0x85, 0x35, 0x02,
    0x93, 0x08, 0x70, 0x00, 0x73, 0x00, 0x00, 0x00, 0x93, 0x08, 0x20, 0x00, 0x73, 0x00, 0x00, 0x00,
    0xef, 0xf0, 0x9f, 0xff, 0x2f, 0x69, 0x6e, 0x69, 0x74, 0x00, 0x00, 0x24, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00,
];

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Context {
    ra: u64,
    sp: u64,

    // callee-saved
    s0: u64,
    s1: u64,
    s2: u64,
    s3: u64,
    s4: u64,
    s5: u64,
    s6: u64,
    s7: u64,
    s8: u64,
    s9: u64,
    s10: u64,
    s11: u64,
}

// Per-CPU state.
pub struct Cpu {
    proc_index: Option<usize>, // The process running on this cpu, or null.
    context: Context,          // swtch() here to enter scheduler().
    noff: i32,                 // Depth of push_off() nesting.
    intena: bool,              // Were interrupts enabled before push_off()?
}

// per-process data for the trap handling code in trampoline.S.
// sits in a page by itself just under the trampoline page in the
// user page table. not specially mapped in the kernel page table.
// the sscratch register points here.
// uservec in trampoline.S saves user registers in the trapframe,
// then initializes registers from the trapframe's
// kernel_sp, kernel_hartid, kernel_satp, and jumps to kernel_trap.
// usertrapret() and userret in trampoline.S set up
// the trapframe's kernel_*, restore user registers from the
// trapframe, switch to the user page table, and enter user space.
// the trapframe includes callee-saved user registers like s0-s11 because the
// return-to-user path via usertrapret() doesn't return through
// the entire kernel call stack.
#[allow(dead_code)]
pub struct Trapframe {
    /*   0 */ kernel_satp: u64, // kernel page table
    /*   8 */ kernel_sp: u64, // top of process's kernel stack
    /*  16 */ kernel_trap: u64, // usertrap()
    /*  24 */ epc: u64, // saved user program counter
    /*  32 */ kernel_hartid: u64, // saved kernel tp
    /*  40 */ ra: u64,
    /*  48 */ sp: u64,
    /*  56 */ gp: u64,
    /*  64 */ tp: u64,
    /*  72 */ t0: u64,
    /*  80 */ t1: u64,
    /*  88 */ t2: u64,
    /*  96 */ s0: u64,
    /* 104 */ s1: u64,
    /* 112 */ a0: u64,
    /* 120 */ a1: u64,
    /* 128 */ a2: u64,
    /* 136 */ a3: u64,
    /* 144 */ a4: u64,
    /* 152 */ a5: u64,
    /* 160 */ a6: u64,
    /* 168 */ a7: u64,
    /* 176 */ s2: u64,
    /* 184 */ s3: u64,
    /* 192 */ s4: u64,
    /* 200 */ s5: u64,
    /* 208 */ s6: u64,
    /* 216 */ s7: u64,
    /* 224 */ s8: u64,
    /* 232 */ s9: u64,
    /* 240 */ s10: u64,
    /* 248 */ s11: u64,
    /* 256 */ t3: u64,
    /* 264 */ t4: u64,
    /* 272 */ t5: u64,
    /* 280 */ t6: u64,
}
#[derive(Clone, Copy)]
enum ProcessState {
    UNUSED,
    USED,
    SLEEPING,
    RUNNABLE,
    RUNNING,
    ZOMBIE,
}

// Per-process state
#[derive(Clone, Copy)]
pub struct Proc {
    // struct spinlock lock;

    // p->lock must be held when using these:
    state: ProcessState, // Process state
    // void *chan;                  // If non-zero, sleeping on chan
    killed: bool, // If non-zero, have been killed
    xstate: i32,  // Exit status to be returned to parent's wait
    pid: i32,     // Process ID

    // wait_lock must be held when using this:
    parent: *mut Proc, // Parent process

    // these are private to the process, so p->lock need not be held.
    kstack: u64,               // Virtual address of kernel stack
    sz: u64,                   // Size of process memory (bytes)
    pagetable: *mut PageTable, // User page table
    trapframe: *mut Trapframe, // data page for trampoline.S
    context: Context,          // swtch() here to run process
    // struct file *ofile[NOFILE];  // Open files
    // struct inode *cwd;           // Current directory
    name: [u8; 16], // Process name (debugging)
}

pub fn procinit() {
    for i in 0..NPROC {
        unsafe {
            proc[i] = RwLock::new(MaybeUninit::zeroed().assume_init());
            proc[i].get_mut().kstack = crate::KSTACK!(i) as u64;
        }
    }
}
//return proc index
pub fn allocproc() -> Option<usize> {
    for i in 0..NPROC {
        unsafe {
            let p = proc[i].get_mut();
            match p.state {
                ProcessState::UNUSED => {
                    p.pid = get_next_pid();
                    p.state = ProcessState::USED;
                    p.trapframe = kalloc() as *mut Trapframe;
                    p.pagetable = proc_pagetable(p);
                    p.context = MaybeUninit::zeroed().assume_init();
                    p.context.ra = forkret as u64;
                    p.context.sp = p.kstack + PGSIZE as u64;
                    return Some(i);
                }
                _ => {
                    break;
                }
            }
        }
    }
    None
}

pub fn proc_pagetable(p: &Proc) -> *mut PageTable {
    let mut pgtable_ptr;
    pgtable_ptr = uvmcreate();

    // map the trampoline code (for system call return)
    // at the highest user virtual address.
    // only the supervisor uses it, on the way
    // to/from user space, so not PTE_U.
    unsafe {
        mappages(
            &mut *pgtable_ptr,
            TRAMPOLINE,
            get_trampoline(),
            PGSIZE,
            PTE_R | PTE_X,
        );
        mappages(
            &mut *pgtable_ptr,
            TRAPFRAME,
            p.trapframe as usize,
            PGSIZE,
            PTE_R | PTE_W,
        );
    }
    pgtable_ptr
}

pub fn freeproc(i: usize) {
    unimplemented!()
}

fn get_next_pid() -> i32 {
    let mut pid = 0;
    unsafe {
        pid = *next_pid.lock();
        *next_pid.lock() += 1;
    }
    pid
}

pub fn forkret() {
    //file system operation not impliment
    usertrapret();
}

pub fn usertrapret() {
    w_stvec((TRAMPOLINE + (get_uservec() - get_trampoline())) as u64);
    //not implement
}

pub fn userinit() {
    let proc_index = allocproc().expect("fiiled to alloc proc");
    unsafe {
        let p = proc[proc_index].get_mut();
        uvminit(&mut *p.pagetable, &initcode);
        p.sz = PGSIZE as u64;
        (*p.trapframe).epc = 0;
        (*p.trapframe).sp = PGSIZE as u64;
        p.state = ProcessState::RUNNABLE;
        slice_cpy(&mut p.name, "initcode".as_bytes());
    }
}

// we do not have mycpu(), because we do not return address of cpu struct.
pub fn cpuid() -> usize {
    return r_tp() as usize;
}

pub fn scheduler() -> ! {
    let cpuid = cpuid();
    unsafe {
        let cpu = cpus[cpuid].get_mut();
        cpu.proc_index = None;
        loop {
            for i in 0..NPROC {
                let p = proc[i].get_mut();
                match p.state {
                    ProcessState::RUNNABLE => {
                        p.state = ProcessState::RUNNING;
                        cpu.proc_index = Some(i);
                        swtch(&mut cpu.context as *mut Context, &mut p.context as *mut Context);
                    }
                    _ => {}
                }
            }
        }
    }
}

extern "C" {
    fn swtch(curr:*mut Context,next: *mut Context);
}