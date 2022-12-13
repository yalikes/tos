use crate::{println, main};

#[no_mangle]
static TIMER_SCRATCH: [u8; 5] = [0; 5];

#[no_mangle]
extern "C" fn start(){
    println!("starting");
    let x: u64 = r_mstatus();
    main();
}

fn r_mstatus() -> u64{
    let x = 42;
    x
}
