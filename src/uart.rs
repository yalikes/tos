use core::fmt;
use spin::Mutex;

use crate::memolayout::UART;
// use lazy_static::lazy_static;
// use uart_16550::MmioSerialPort;
// use spin::Mutex;
// use crate::memolayout;

// lazy_static!{
//     pub static ref TERMINAL_WRITER: Mutex<MmioSerialPort> = Mutex::new(
//        unsafe { MmioSerialPort::new(memolayout::UART)
//     });
// }

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::uart::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    get_uart_ref().write_fmt(args).unwrap();
}

const IER_RX_ENABLE: u8 = 1 << 0;
const IER_TX_ENABLE: u8 = 1 << 1;

const FCR_FIFO_ENABLE: u8 = 1 << 0;
const FCR_FIFO_CLEAR: u8 = 3 << 1;

const LCR_EIGHT_BITS: u8 = 3 << 0;
const LCR_BAUD_LATCH: u8 = 1 << 7;

const LSR_RX_READY: u8 = 1 << 0;
const LSR_TX_IDLE: u8 = 1 << 5;
static mut SERIAL_PORT: Mutex<*mut UartMimo> = Mutex::new(UART as *mut UartMimo);

struct UartMimo {
    rhr_thr: u8,
    ier: u8,
    fcr_isr: u8,
    lcr: u8,
    __padding: u8,
    lsr: u8,
}

pub fn console_init() {
    uart_init();
}

fn get_uart_ref<'a>() -> &'a mut UartMimo {
    let mut uart_ptr = unsafe { SERIAL_PORT.lock() };
    unsafe { &mut **uart_ptr }
}

fn uart_init() {
    let uart_ref = get_uart_ref();
    uart_ref.ier = 0; //disable interrupts
    uart_ref.lcr = LCR_BAUD_LATCH;
    uart_ref.fcr_isr = 0x03; //LSB for baud rate 38.4k
    uart_ref.ier = 0x00; //MSB for baud rate 38.4k

    uart_ref.lcr = LCR_EIGHT_BITS; //leave set-baud mode and set word length to 8 bit
    uart_ref.fcr_isr = FCR_FIFO_ENABLE | FCR_FIFO_CLEAR; //reset and clear FIFOs
    uart_ref.ier = IER_TX_ENABLE | IER_RX_ENABLE; //enable transmit and receive interrupts
}

impl UartMimo {
    fn _write_char(&mut self, c: u8) {
        while self.lsr & LSR_TX_IDLE == 0 {}
        self.rhr_thr = c;
    }
}

pub fn uartputc_sync(c: u8) {
    let uart_ref = get_uart_ref();
    while uart_ref.lsr & LSR_TX_IDLE == 0 {}
    uart_ref.rhr_thr = c;
}

impl fmt::Write for UartMimo {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            self._write_char(c);
        }
        Ok(())
    }
}

pub fn uart_intr() {
    loop {
        let c_opt = uart_getc();
        match c_opt {
            None => break,
            Some(c) => console_intr(c),
        }
    }
}

pub fn uart_getc() -> Option<u8> {
    let uart_ref = get_uart_ref();
    if uart_ref.lsr & 0x01 != 0 {
        // input data is ready
        Some(uart_ref.rhr_thr)
    } else {
        None
    }
}

fn console_intr(c: u8) {
    uartputc_sync(c);
}
