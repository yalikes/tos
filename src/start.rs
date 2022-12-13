use crate::{main, println};
use crate::riscv::{r_mstatus, w_mstatus, };

#[no_mangle]
static TIMER_SCRATCH: [u8; 5] = [0; 5];

#[no_mangle]
extern "C" fn start() {
    // set M Previous Privilege mode to Supervisor, for mret.
    println!("starting");
    let x: u64 = r_mstatus();
    main();
}



