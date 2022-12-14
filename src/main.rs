#![no_std]
#![no_main]

mod memolayout;
mod uart;
mod start;
mod riscv;

use core::{arch::global_asm, panic::PanicInfo};
use linked_list_allocator::LockedHeap;
use alloc::{vec, vec::Vec};

extern crate alloc;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("trampoline.asm"));

#[no_mangle]
static STACK0: [u8; 4096] = [0; 4096];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn main() -> !{
    let heap_start = crate::memolayout::get_kernel_end();
    let heap_end = crate::memolayout::PHYSTOP;
    let heap_size = heap_end - heap_start;
    unsafe{
        ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
    let mut v = Vec::new();
    v.push(12);
    loop {
    }

}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
