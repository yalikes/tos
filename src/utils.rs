use crate::{println, print};
pub fn print_addr(addr: u64, size: usize){
    for i in 0..size{
        if i % 16 == 0{
            println!();
        }
        print!("{:02x} ", unsafe { *((addr + i as u64) as *const u8)});
    }
}

pub fn get_ref_addr<T>(v: &T) -> u64{
    v as *const T as u64
}