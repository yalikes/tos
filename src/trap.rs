use crate::memolayout::get_kernelvec;
use crate::riscv::w_stvec;

// set up to take exceptions and traps while in the kernel.
pub fn trapinithart()
{
  w_stvec(get_kernelvec() as u64);
}