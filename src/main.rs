#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

mod memolayout;
mod riscv;
mod start;
mod uart;
mod vm;
mod utils;

use core::{arch::global_asm, panic::PanicInfo};
use linked_list_allocator::LockedHeap;

extern crate alloc;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("trampoline.asm"));
global_asm!(include_str!("kernelvec.asm"));

#[no_mangle]
static STACK0: [u8; 4096] = [0; 4096];

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[no_mangle]
pub extern "C" fn main() -> ! {
    let heap_start = crate::memolayout::get_kernel_end();
    let heap_end = crate::memolayout::PHYSTOP;
    let heap_size = heap_end - heap_start;
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }
    vm::kvminit();
    vm::kvminithart();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout);
}
