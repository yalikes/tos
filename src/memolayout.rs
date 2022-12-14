pub const UART: usize = 0x1000_0000;
pub const KERNELBASE: usize = 0x8000_0000;
pub const PHYSTOP: usize = KERNELBASE + 128 * 1024 * 1024;

extern "C" {
    pub static end: u8;
}

pub fn get_kernel_end() -> usize{
    unsafe {
        return (&end as *const u8) as usize;
    }
}
