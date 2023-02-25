use crate::{proc::{procid, proc}, uart::uartputc_sync};
pub fn syscall(){
    let proc_index = procid().unwrap();
    unsafe{
        let proc_guard = proc[proc_index].read();
        let trapfram = &mut (*proc_guard.trapframe);
        let num = trapfram.a7;
        if num == 114{
            uartputc_sync(65);
            // print!("a");
        }
    }
}
