#![no_std]
#![no_main]

use core::{arch::global_asm, panic::PanicInfo};

global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub extern "C" fn start() -> !{
    loop {}
}

//global_asm!(include_str!("entry.asm"));

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
