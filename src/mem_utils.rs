use core::cmp::min;

pub unsafe fn memmove(mem: *mut u8,src: *const u8,sz: usize){
    for i in 0..sz{
        *((mem as usize + i) as *mut u8) = *((src as usize + i) as *const u8);
    }
}

pub fn slice_cpy<T: Copy>(dst:&mut [T], src: &[T]){
    let len = min(dst.len(), src.len());
    dst[..len].copy_from_slice(&src[..len]);
}

pub unsafe fn memset(mem: *mut u8, val: u8, size: usize){
    for i in 0..size{
        *((mem as usize +i) as *mut u8) = val;
    }
}