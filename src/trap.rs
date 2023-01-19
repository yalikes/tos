use core::panic;

use crate::MAKE_SATP;
use crate::memolayout::{get_kernelvec, get_trampoline, get_uservec, get_userret,TRAMPOLINE, TRAPFRAME};
use crate::proc::{proc, procid, Trapframe};
use crate::riscv::{
    r_satp, r_sstatus, r_tp, w_sepc, w_sstatus, w_stvec,
    PGSIZE, SSTATUS_SPIE, SSTATUS_SPP, SATP_SV39, r_sepc, r_scause
};
use crate::syscall::syscall;

// set up to take exceptions and traps while in the kernel.
pub fn trapinithart() {
    w_stvec(get_kernelvec() as u64);
}

pub fn usertrapret() {
    // we're about to switch the destination of traps from
    // kerneltrap() to usertrap(), so turn off interrupts until
    // we're back in user space, where usertrap() is correct.
    // intr_off();
    //
    let proc_index = procid().unwrap();
    let mut p = unsafe { proc[proc_index].write() };
    w_stvec((TRAMPOLINE + (get_uservec() - get_trampoline())) as u64);
    //not implement
    unsafe {
        let trapframe = &mut (*p.trapframe);
        trapframe.kernel_satp = r_satp(); // kernel page table
        trapframe.kernel_sp = p.kstack + PGSIZE as u64; // process's kernel stack
        trapframe.kernel_trap = usertrap as u64;
        trapframe.kernel_hartid = r_tp(); // hartid for cpuid()
    }
    // set up the registers that trampoline.asm's sret will use
    // to get to user space.

    // set S Previous Privilege mode to User.
    let mut x = r_sstatus();
    x &= !SSTATUS_SPP; // clear SPP to 0 for user mode
    x |= SSTATUS_SPIE; // enable interrupts in user mode
    w_sstatus(x);
    // set S Exception Program Counter to the saved user pc.
    let epc = unsafe { (*p.trapframe).epc };
    w_sepc(epc);
    let satp = MAKE_SATP!(unsafe{(&(*p.pagetable).ptes as *const [u64; 512]) as u64});
    drop(p);
    let fn_ptr_addr = (TRAMPOLINE + (get_userret() - get_trampoline())) as *const ();
    let fn_ptr: extern "C" fn(u64, u64) -> () = unsafe{ core::mem::transmute(fn_ptr_addr)};
    fn_ptr(TRAPFRAME as u64, satp);
}

pub fn usertrap() {
    // when a exception occurs, before we disable exception, another excpetion occurs
    // does this program can handle this?
    if (r_sstatus() & SSTATUS_SPP) != 0{
        panic!("usertrap: not from user mode");
    }
    // send interrupts and exceptions to kerneltrap(),
    // since we're now in the kernel.
    w_stvec(get_kernelvec() as u64);
    let proc_index = procid().unwrap();
    unsafe{
        let mut proc_guard = proc[proc_index].write();
        let trapfram: &mut Trapframe = &mut (*proc_guard.trapframe);
        trapfram.epc = r_sepc();
        if r_scause() == 8{
            //syscall
            if proc_guard.killed {
                //process killed
                loop {}
            }
            // sepc points to the ecall instruction,
            // but we want to return to the next instruction.
            trapfram.epc += 4;
            
            // an interrupt will change sstatus &c registers,
            // so don't enable until done with those registers.
            drop(proc_guard);
            syscall();
        }
    }
    usertrapret();
}
