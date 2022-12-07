#![no_std]
#![no_main]

mod memolayout;
mod uart;

use core::{arch::global_asm, panic::PanicInfo};



global_asm!(include_str!("entry.asm"));

#[no_mangle]
static STACK0: [u8; 4096] = [0; 4096];

#[no_mangle]
pub extern "C" fn start() -> !{
    println!("Hello World!");
    loop {
    }

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
