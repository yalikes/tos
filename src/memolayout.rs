pub const UART: usize = 0x1000_0000;
pub const KERNELBASE: usize = 0x8000_0000;
pub const PHYSTOP: usize = KERNELBASE + 128 * 1024 * 1024;
pub const CLINT: usize = 0x200_0000;
pub const CLINT_MTIME: usize = CLINT + 0xBFF8;

extern "C" {
    static end: u8;
    static etext: u8;
}

pub fn get_kernel_end() -> usize{
    unsafe {
        return (&end as *const u8) as usize;
    }
}

pub fn get_etext() -> usize{
    unsafe {
        return (&etext as *const u8) as usize;
    }
}

#[inline]
pub fn CLINT_MTIMECMP(hartid: u64) -> u64{
    return (CLINT as u64) + 0x4000 + 8*hartid;
}

