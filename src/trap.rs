use core::panic;

use crate::memolayout::{
    get_kernelvec, get_trampoline, get_userret, get_uservec, TRAMPOLINE, TRAPFRAME, UART_IRQ,
    VIRTIO0_IRQ,
};
use crate::plic::{plic_claim, plic_complete};
use crate::proc::{proc, procid, Trapframe, myproc};
use crate::riscv::{
    intr_get, intr_off, intr_on, r_satp, r_scause, r_sepc, r_sstatus, r_stval, r_tp, w_sepc,
    w_sstatus, w_stvec, PGSIZE, SATP_SV39, SSTATUS_SPIE, SSTATUS_SPP,
};
use crate::syscall::syscall;
use crate::uart::uart_intr;
use crate::virtio::virtio_blk::virtio_disk_intr;
use crate::{println, MAKE_SATP};

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
    let mut p = unsafe { &mut proc[proc_index] };
    intr_off();
    w_stvec((TRAMPOLINE + (get_uservec() - get_trampoline())) as u64);
    //not implement
    unsafe {
        let trapframe = &mut (*p.trapframe);
        trapframe.kernel_satp = r_satp(); // kernel page table
        trapframe.kernel_sp = p.kstack + (PGSIZE as u64) * 15 as u64; // process's kernel stack
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
    let satp = MAKE_SATP!(unsafe { (&(*p.pagetable).ptes as *const [u64; 512]) as u64 });
    drop(p);
    let fn_ptr_addr = (TRAMPOLINE + (get_userret() - get_trampoline())) as *const ();
    let fn_ptr: extern "C" fn(u64, u64) -> () = unsafe { core::mem::transmute(fn_ptr_addr) };
    fn_ptr(TRAPFRAME as u64, satp);
}

pub fn usertrap() {
    let mut intr_type = DevintrState::NotRecognized;
    let mut proc_killed = false;
    // when a exception occurs, before we disable exception, another excpetion occurs
    // does this program can handle this?
    if (r_sstatus() & SSTATUS_SPP) != 0 {
        panic!("usertrap: not from user mode");
    }
    // send interrupts and exceptions to kerneltrap(),
    // since we're now in the kernel.
    w_stvec(get_kernelvec() as u64);
    let proc_index = procid().unwrap();
    let mut p = unsafe { &mut proc[proc_index] };
    let trapfram: &mut Trapframe = unsafe { &mut (*p.trapframe) };
    trapfram.epc = r_sepc();
    if r_scause() == 8 {
        //syscall
        proc_killed = p.killed;
        if proc_killed {
            //process killed
            loop {}
        }
        // sepc points to the ecall instruction,
        // but we want to return to the next instruction.
        trapfram.epc += 4;

        // an interrupt will change sstatus &c registers,
        // so don't enable until done with those registers.
        drop(p);
        intr_on();
        syscall();
    } else {
        intr_type = devintr();
        match intr_type {
            DevintrState::NotRecognized => {
                println!("usertrap(): unexpected scause {} pid={}", r_scause(), p.pid);
                println!("            sepc={} stval={}", r_sepc(), r_stval());
                p.killed = true;
                proc_killed = p.killed;
            }
            _ => {}
        }
    }
    if proc_killed {
        //not implement
    }
    if matches!(intr_type, DevintrState::TimerIntr) {
        // yield, but not implememnt
    }
    usertrapret();
}

#[no_mangle]
pub fn kerneltrap() {
    let intr_type;
    let sepc = r_sepc();
    let sstatus = r_sstatus();
    let scause = r_scause();

    if sstatus & SSTATUS_SPP == 0 {
        panic!("kerneltrap: not from supervisor mode");
    }
    if intr_get() {
        panic!("kerneltrap: interrupts ");
    }
    intr_type = devintr();
    if matches!(intr_type, DevintrState::NotRecognized) {
        println!("scause {}", scause);
        println!("sepc={} stval={}", r_sepc(), r_stval());
        panic!("kerneltrap");
    }
    if matches!(intr_type, DevintrState::TimerIntr) {
        // need yield cpu here
    }

    w_sepc(sepc);
    w_sstatus(sstatus);
}

enum DevintrState {
    TimerIntr,
    OtherDev,
    NotRecognized,
}

// check if it's an external interrupt or software interrupt,
// and handle it.
fn devintr() -> DevintrState {
    let scause = r_scause();
    if (scause & 0x8000000000000000) != 0 && (scause & 0xff) == 9 {
        // this is a supervisor external interrupt, via PLIC.

        // irq indicates which device interrupted.
        let irq = plic_claim();
        if irq == VIRTIO0_IRQ as u32 {
            virtio_disk_intr();
        } else if irq == UART_IRQ as u32 {
            uart_intr();
            // println!("unexpected interrupt irq={irq}");
        }
        // the PLIC allows each device to raise at most one
        // interrupt at a time; tell the PLIC the device is
        // now allowed to interrupt again.
        if irq != 0 {
            plic_complete(irq);
        }
        return DevintrState::OtherDev;
    }
    return DevintrState::NotRecognized;
}
