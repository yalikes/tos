use crate::riscv::{MAXVA, PGSIZE};

// You can found those address by dump qemu dtb, and use dtc to get dts file,
// which describe virtual machine's memory layout.
// we manually write this for simplicity.
pub const UART: usize = 0x1000_0000;
pub const KERNELBASE: usize = 0x8000_0000;
pub const PHYSTOP: usize = KERNELBASE + 128 * 1024 * 1024;
pub const CLINT: usize = 0x200_0000;
pub const CLINT_MTIME: usize = CLINT + 0xBFF8;
pub const TRAMPOLINE: usize = MAXVA as usize - PGSIZE;
pub const TRAPFRAME: usize = TRAMPOLINE - PGSIZE;
// virtio mmio interface
pub const VIRTIO0: usize = 0x10001000;
pub const VIRTIO0_IRQ: usize = 1;
extern "C" {
    static end: u8;
    static etext: u8;
    fn kernelvec();
    fn trampoline();
    fn uservec();
    fn userret();
}

pub fn get_kernel_end() -> usize {
    unsafe {
        return (&end as *const u8) as usize;
    }
}

pub fn get_etext() -> usize {
    unsafe {
        return (&etext as *const u8) as usize;
    }
}
pub fn get_kernelvec() -> usize {
    return kernelvec as usize;
}

pub fn get_trampoline() -> usize {
    return trampoline as usize;
}

pub fn get_uservec() -> usize {
    return uservec as usize;
}

pub fn get_userret() -> usize {
    return userret as usize;
}

#[inline]
pub fn clint_mtimecmp(hartid: u64) -> u64 {
    return (CLINT as u64) + 0x4000 + 8 * hartid;
}

#[macro_export]
macro_rules! KSTACK {
    ($p: expr) => {
        TRAMPOLINE - ($p + 1) * 2 * PGSIZE
    };
}
