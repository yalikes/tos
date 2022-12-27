use core::fmt;
use lazy_static::lazy_static;
use uart_16550::MmioSerialPort;
use spin::Mutex;
use crate::memolayout;

lazy_static!{
    pub static ref TERMINAL_WRITER: Mutex<MmioSerialPort> = Mutex::new(
        unsafe { MmioSerialPort::new(memolayout::UART)
    });
}


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
pub fn _print(args: fmt::Arguments){
    use core::fmt::Write;
    TERMINAL_WRITER.lock().write_fmt(args).unwrap();
}