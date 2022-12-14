use core::arch::asm;

pub const MSTATUS_MPP_MASK: u64 = 3 << 11;
pub const MSTATUS_MPP_M: u64 = 3 << 11;
pub const MSTATUS_MPP_S: u64 = 1 << 11;
pub const MSTATUS_MPP_U: u64 = 0 << 11;
pub const MSTATUS_MIE: u64 = 1 << 3;

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

#[inline]
pub fn w_mepc(x: u64){
    unsafe {
        asm! {
            "csrw mepc, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Supervisor Status Register, sstatus
pub const SSTATUS_SSP: u64 = 1<<8;
pub const SSTATUS_SPIE: u64 = 1<<5;
pub const SSTATUS_UPIE: u64 = 1<<4;
pub const SSTATUS_SIE: u64 = 1<<1;
pub const SSTATUS_UIE: u64 = 1<<0;



#[inline]
pub fn r_sstatus() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, sstatus",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_sstatus(x: u64){
    unsafe {
        asm! {
            "csrw sstatus, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Supervisor Interrupt Pending
#[inline]
pub fn r_sip() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, sip",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_sip(x: u64){
    unsafe {
        asm! {
            "csrw sip, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

//Supervisor Interupt Enable
pub const SIE_SEIE: u64 = 1<<9;//external
pub const SIE_STIE: u64 = 1<<5;//timer
pub const SIE_SSIE: u64 = 1<<1;//software

#[inline]
pub fn r_sie() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, sie",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_sie(x: u64){
    unsafe {
        asm! {
            "csrw sie, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Machine-mode Interrupt Enable
pub const MIE_SEIE: u64 = 1<<9;//external
pub const MIE_STIE: u64 = 1<<5;//timer
pub const MIE_SSIE: u64 = 1<<1;//software

#[inline]
pub fn r_mie() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, mie",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_mie(x: u64){
    unsafe {
        asm! {
            "csrw mie, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// supervisor exception program counter, holds the
// instruction address to which a return from
// exception will go.
#[inline]
pub fn r_sepc() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, sepc",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_sepc(x: u64){
    unsafe {
        asm! {
            "csrw sepc, {x}",
            x = in(reg) x
        } //volatile by default
    }
}