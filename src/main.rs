#![no_std]
#![no_main]
mod memolayout;

use core::{arch::global_asm, panic::PanicInfo};
use uart_16550::MmioSerialPort;


global_asm!(include_str!("entry.asm"));

#[no_mangle]
static STACK0: [u8; 4096] = [0; 4096];

#[no_mangle]
pub extern "C" fn start() -> !{
    let mut serial_port = unsafe { MmioSerialPort::new(memolayout::UART)};
    serial_port.init();

    for i in 0..128{
        serial_port.send(i);
    }
    loop {
        let x = serial_port.receive();
        serial_port.send(x);
        serial_port.send(x);
    }

}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
