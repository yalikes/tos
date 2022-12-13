use crate::{main, println};
use core::arch::asm;

#[no_mangle]
static TIMER_SCRATCH: [u8; 5] = [0; 5];

#[no_mangle]
extern "C" fn start() {
    // set M Previous Privilege mode to Supervisor, for mret.
    println!("starting");
    let x: u64 = r_mstatus();
    main();
}

fn r_mstatus() -> u64 {
    let mut x: u64;
    unsafe {
        asm! {
            "csrr {x}, mstatus",
            x = out(reg) x
        } //volatile by default
    }
    x
}
