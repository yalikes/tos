use crate::{main, println};
use crate::riscv::{r_mstatus, w_mstatus, w_mepc, MSTATUS_MPP_MASK, MSTATUS_MPP_S};

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
}



