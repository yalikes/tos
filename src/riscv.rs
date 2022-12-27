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
pub const MIE_MEIE: u64 = 1<<11;//external
pub const MIE_MTIE: u64 = 1<<7;//timer
pub const MIE_MSIE: u64 = 1<<3;//software

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

// Machine Exception Delegation
#[inline]
pub fn r_medeleg() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, medeleg",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_medeleg(x: u64){
    unsafe {
        asm! {
            "csrw medeleg, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Machine Interrupt Delegation
#[inline]
pub fn r_mideleg() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, mideleg",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_mideleg(x: u64){
    unsafe {
        asm! {
            "csrw mideleg, {x}",
            x = in(reg) x
        } //volatile by default
    }
}
// Supervisor Trap-Vector Base Address
// low two bits are mode.
#[inline]
pub fn r_stvec() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, stvec",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_stvec(x: u64){
    unsafe {
        asm! {
            "csrw stvec, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Machine-mode interrupt vector
#[inline]
pub fn w_mtvec(x: u64){
    unsafe {
        asm! {
            "csrw mtvec, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

#[inline]
pub fn w_pmpcfg0(x: u64){
    unsafe {
        asm! {
            "csrw pmpcfg0, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

#[inline]
pub fn w_pmpaddr0(x: u64){
    unsafe {
        asm! {
            "csrw pmpaddr0, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// use riscv's sv39 page table scheme.
pub const SATP_SV39: u64 = 8<<60;

//#define MAKE_SATP(pagetable) (SATP_SV39 | (((uint64)pagetable) >> 12))
//I don't what this does

#[inline]
pub fn r_satp() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, satp",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_satp(x: u64){
    unsafe {
        asm! {
            "csrw satp, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Supervisor Scratch register, for early trap handler in trampoline.S.
#[inline]
pub fn w_sscratch(x: u64){
    unsafe {
        asm! {
            "csrw sscratch, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

#[inline]
pub fn w_mscratch(x: u64){
    unsafe {
        asm! {
            "csrw mscratch, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

// Supervisor Trap Cause
#[inline]
pub fn r_scause() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, scause",
            x = out(reg) x
        } //volatile by default
    }
    x
}

// Supervisor Trap Value
#[inline]
pub fn r_stval() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, stval",
            x = out(reg) x
        } //volatile by default
    }
    x
}

// Machine-mode Counter-Enable
#[inline]
pub fn w_mcounteren(x: u64){
    unsafe {
        asm! {
            "csrw mcounteren, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

#[inline]
pub fn r_mcounteren() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, mcounteren",
            x = out(reg) x
        } //volatile by default
    }
    x
}

// machine-mode cycle counter
#[inline]
pub fn r_time() -> u64{
    let mut x;
    unsafe {
        asm! {
            "csrr {x}, time",
            x = out(reg) x
        } //volatile by default
    }
    x
}

// enable device interrupts
#[inline]
pub fn intr_on(){
    w_sstatus(r_sstatus() | SSTATUS_SIE);
}

// disable device interrupts
#[inline]
pub fn intr_off(){
    w_sstatus(r_sstatus() & !SSTATUS_SIE);
}

// are device interrupts enabled?
#[inline]
pub fn intr_get() -> bool{
    let x = r_sstatus();
    return (x & SSTATUS_SIE) != 0;
}

#[inline]
pub fn r_sp() -> u64{
    let mut x;
    unsafe {
        asm! {
            "mv {x}, sp",
            x = out(reg) x
        } //volatile by default
    }
    x
}

// read and write tp, the thread pointer, which holds
// this core's hartid (core number), the index into cpus[].
#[inline]
pub fn r_tp() -> u64{
    let mut x;
    unsafe {
        asm! {
            "mv {x}, tp",
            x = out(reg) x
        } //volatile by default
    }
    x
}

#[inline]
pub fn w_tp(x: u64){
    unsafe {
        asm! {
            "mv tp, {x}",
            x = in(reg) x
        } //volatile by default
    }
}

#[inline]
pub fn r_ra() -> u64{
    let mut x;
    unsafe {
        asm! {
            "mv {x}, ra",
            x = out(reg) x
        } //volatile by default
    }
    x
}

// flush the TLB.
#[inline]
pub fn sfence_vma(){
    unsafe {
        asm! {
            // the zero, zero means flush all TLB entries.
            "sfence.vma zero, zero"
        } //volatile by default
    }
}

pub const PGSIZE: usize = 4096;
pub const PGSHIFT: usize = 12;

pub const PTE_V: u64 = 1 << 0; // valid
pub const PTE_R: u64 = 1 << 1;
pub const PTE_W: u64 = 1 << 2;
pub const PTE_X: u64 = 1 << 3;
pub const PTE_U: u64 = 1 << 4; // 1 -> user can access

pub const MAXVA: u64 = 1 << (9 + 9 + 9 + 12 -1);


// extract the three 9-bit page table indices from a virtual address.
pub const PXMASK: u64 = 0x1FF; // 9bits
#[macro_export]
macro_rules! PGROUNDDOWN {
    ($exp: expr) => {
        $exp & !(PGSIZE - 1)
    };
}
#[macro_export]
macro_rules! PX {
    ($level: expr, $va: expr) => {
        ($va >> (PGSHIFT + ($level * 9))) & PXMASK as usize
    };
}

// shift a physical address to the right place for a PTE.
#[macro_export]
macro_rules! PTE2PA {
    ($pte: expr) => {
        ($pte >> 10) << 12
    };
}
#[macro_export]
macro_rules! PA2PTE {
    ($pa: expr) => {
        ($pa >> 12) << 10
    };
}

#[macro_export]
macro_rules! MAKE_SATP {
    ($pgtbl_addr:expr) => {
        SATP_SV39 | ($pgtbl_addr as u64 >> 12);
    };
}