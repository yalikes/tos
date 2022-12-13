use core::arch::asm;

pub const MSTATUS_MPP_MASK:u64 = 3 << 11;
pub const MSTATUS_MPP_M:u64 = 3 << 11;
pub const MSTATUS_MPP_S:u64 = 1 << 11;
pub const MSTATUS_MPP_U:u64 = 0 << 11;
pub const MSTATUS_MIE:u64 = 1 << 3;

#[inline]
pub fn r_mhartid() -> u64 {
    let mut x: u64;
    unsafe {
        asm! {
            "csrr {x}, mhartid",
            x = out(reg) x
        }
    }
    x
}

#[inline]
pub fn r_mstatus() -> u64 {
    let mut x: u64;
    unsafe {
        asm! {
            "csrr {x}, mstatus",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_mstatus(x: u64) {
    unsafe {
        asm! {
            "csrw mstatus, {x}",
            x = in(reg) x
        } //volatile by default
    }
}