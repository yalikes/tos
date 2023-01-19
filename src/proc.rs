use core::mem::MaybeUninit;
use spin::{Mutex, RwLock};

use crate::mem_utils::slice_cpy;
use crate::memolayout::{get_trampoline, TRAMPOLINE, TRAPFRAME};
use crate::params::{NCPU, NPROC};
use crate::riscv::{r_tp, PGSIZE, PTE_R, PTE_W, PTE_X};
use crate::vm::{kalloc, mappages, uvmcreate, uvminit, PageTable};
use crate::trap::usertrapret;

// Saved registers for kernel context switches.

pub static mut next_pid: Mutex<i32> = Mutex::new(1);

pub static mut proc: [RwLock<Proc>; NPROC] = unsafe { MaybeUninit::zeroed().assume_init() }; // because this is convient
pub static mut cpus: [RwLock<Cpu>; NCPU] = unsafe { MaybeUninit::zeroed().assume_init() };


/// riscv64-linux-gnu-gcc -c initcode.S  -o initcode.o
/// riscv64-linux-gnu-objcopy -S -O binary initcode.o initcode
/// od -t xC initcode 
/// copy content of initcode into here
// with loop
// pub static initcode: [u8; 64] = [
// 0x17, 0x05, 0x00, 0x00, 0x03, 0x35, 0x05, 0x00, 0x97, 0x05, 0x00, 0x00, 0x83, 0xb5, 0x05, 0x00,
// 0x9d, 0x48, 0x97, 0x00, 0x00, 0x00, 0xe7, 0x80, 0x00, 0x00, 0x73, 0x00, 0x00, 0x00, 0x01, 0xa0,
// 0x93, 0x08, 0xa0, 0x02, 0x73, 0x00, 0x00, 0x00, 0xef, 0xf0, 0x9f, 0xff, 0x2f, 0x69, 0x6e, 0x69,
// 0x74, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
// ];

//without loop
//pub static initcode: [u8; 56] = [
//0x17, 0x05, 0x00, 0x00, 0x03, 0x35, 0x05, 0x00, 0x97, 0x05, 0x00, 0x00, 0x83, 0xb5, 0x05, 0x00,
//0x9d, 0x48, 0x73, 0x00, 0x00, 0x00, 0x01, 0xa0, 0x93, 0x08, 0xa0, 0x02, 0x73, 0x00, 0x00, 0x00,
//0xef, 0xf0, 0x9f, 0xff, 0x2f, 0x69, 0x6e, 0x69, 0x74, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00,
//0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
//];
pub static initcode: [u8; 60] = [
0x17, 0x05, 0x00, 0x00, 0x03, 0x35, 0x05, 0x00, 0x97, 0x05, 0x00, 0x00, 0x83, 0xb5, 0x05, 0x00,
0x93, 0x08, 0x20, 0x07, 0x09, 0xa0, 0x73, 0x00, 0x00, 0x00, 0xf5, 0xbf, 0x93, 0x08, 0xa0, 0x02,
0x73, 0x00, 0x00, 0x00, 0xef, 0xf0, 0x9f, 0xff, 0x2f, 0x69, 0x6e, 0x69, 0x74, 0x00, 0x00, 0x01,
0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Context {
    pub ra: u64,
    pub sp: u64,

    // callee-saved
    pub s0: u64,
    pub s1: u64,
    pub s2: u64,
    pub s3: u64,
    pub s4: u64,
    pub s5: u64,
    pub s6: u64,
    pub s7: u64,
    pub s8: u64,
    pub s9: u64,
    pub s10: u64,
    pub s11: u64,
}

// Per-CPU state.
pub struct Cpu {
    pub proc_index: Option<usize>, // The process running on this cpu, or null.
    pub context: Context,          // swtch() here to enter scheduler().
    pub noff: i32,                 // Depth of push_off() nesting.
    pub intena: bool,              // Were interrupts enabled before push_off()?
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
    /*   0 */ pub kernel_satp: u64, // kernel page table
    /*   8 */ pub kernel_sp: u64, // top of process's kernel stack
    /*  16 */ pub kernel_trap: u64, // usertrap()
    /*  24 */ pub epc: u64, // saved user program counter
    /*  32 */ pub kernel_hartid: u64, // saved kernel tp
    /*  40 */ pub ra: u64,
    /*  48 */ pub sp: u64,
    /*  56 */ pub gp: u64,
    /*  64 */ pub tp: u64,
    /*  72 */ pub t0: u64,
    /*  80 */ pub t1: u64,
    /*  88 */ pub t2: u64,
    /*  96 */ pub s0: u64,
    /* 104 */ pub s1: u64,
    /* 112 */ pub a0: u64,
    /* 120 */ pub a1: u64,
    /* 128 */ pub a2: u64,
    /* 136 */ pub a3: u64,
    /* 144 */ pub a4: u64,
    /* 152 */ pub a5: u64,
    /* 160 */ pub a6: u64,
    /* 168 */ pub a7: u64,
    /* 176 */ pub s2: u64,
    /* 184 */ pub s3: u64,
    /* 192 */ pub s4: u64,
    /* 200 */ pub s5: u64,
    /* 208 */ pub s6: u64,
    /* 216 */ pub s7: u64,
    /* 224 */ pub s8: u64,
    /* 232 */ pub s9: u64,
    /* 240 */ pub s10: u64,
    /* 248 */ pub s11: u64,
    /* 256 */ pub t3: u64,
    /* 264 */ pub t4: u64,
    /* 272 */ pub t5: u64,
    /* 280 */ pub t6: u64,
}
#[derive(Clone, Copy)]
pub enum ProcessState {
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
    pub state: ProcessState, // Process state
    // void *chan;                  // If non-zero, sleeping on chan
    pub killed: bool, // If non-zero, have been killed
    pub xstate: i32,  // Exit status to be returned to parent's wait
    pub pid: i32,     // Process ID

    // wait_lock must be held when using this:
    pub parent: *mut Proc, // Parent process

    // these are private to the process, so p->lock need not be held.
    pub kstack: u64,               // Virtual address of kernel stack
    pub sz: u64,                   // Size of process memory (bytes)
    pub pagetable: *mut PageTable, // User page table
    pub trapframe: *mut Trapframe, // data page for trampoline.S
    pub context: Context,          // swtch() here to run process
    // struct file *ofile[NOFILE];  // Open files
    // struct inode *cwd;           // Current directory
    pub name: [u8; 16], // Process name (debugging)
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
    let pgtable_ptr;
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

#[allow(unused_variables)]
pub fn freeproc(i: usize) {
    unimplemented!()
}

fn get_next_pid() -> i32 {
    let pid;
    unsafe {
        let mut next_pid_guard = next_pid.lock();
        pid = *next_pid_guard;
        *next_pid_guard += 1;
    }
    pid
}

pub fn forkret() {
    //file system operation not implement
    usertrapret();
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

pub fn procid() -> Option<usize> {
    //push_off
    let cpuid = cpuid();
    let procid = unsafe { cpus[cpuid].read().proc_index };

    //pop_off
    procid
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
                        swtch(
                            &mut cpu.context as *mut Context,
                            &mut p.context as *mut Context,
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}

extern "C" {
    fn swtch(curr: *mut Context, next: *mut Context);
}
