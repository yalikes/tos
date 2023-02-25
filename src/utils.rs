pub fn print_addr(addr: u64, size: usize){
    for i in 0..size{
        if i % 16 == 0{
            // println!();
        }
        // print!("{:02x} ", unsafe { *((addr + i as u64) as *const u8)});
    }
}