use core::arch::asm;

use crate::{main, println};
use crate::riscv::*;

#[no_mangle]
static TIMER_SCRATCH: [u8; 5] = [0; 5];

#[no_mangle]
extern "C" fn start() {
    // set M Previous Privilege mode to Supervisor, for mret.
    println!("starting");
    let mut x: u64 = r_mstatus();
    x &= !MSTATUS_MPP_MASK;
    x |= MSTATUS_MPP_S;
    w_mstatus(x);

    //set M Exception Program Counter to main, for mret
    w_mepc(main as u64);
    w_satp(0);
    // delegate all interrupts and exceptions to supervisor mode.
    w_medeleg(0xffff);
    w_mideleg(0xffff);
    w_sie(r_sie() | SIE_SEIE | SIE_STIE | SIE_SSIE);
    // configure Physical Memory Protection to give supervisor mode
    // access to all of physical memory.
    w_pmpaddr0(0x3fffffffffffff);
    w_pmpcfg0(0xf);
    timerinit();
    let id = r_mhartid();
    w_tp(id);
    unsafe{asm!("mret");}
}


fn timerinit(){
    let id = r_mhartid();
    let interval = 1000000; // cycles; about 1/10th second in qemu.


}

