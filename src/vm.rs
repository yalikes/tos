use core::alloc::Layout;
use core::panic;

use crate::memolayout::{get_etext, get_trampoline, KERNELBASE, PHYSTOP, TRAMPOLINE, UART};
use crate::{riscv::*, ALLOCATOR};
use crate::{MAKE_SATP, PA2PTE, PGROUNDDOWN, PTE2PA, PX};
#[repr(C)]
pub struct PageTable {
    ptes: [u64; 512],
}
// struct

pub static mut KERN_PG_ADDR: *mut PageTable = 0 as *mut PageTable;

// pub static mut kernel_memory_list;

//we need make our kernel to use direct map
pub fn kvminit() {
    unsafe {
        KERN_PG_ADDR = kalloc() as *mut PageTable;
        kvmmake(&mut *KERN_PG_ADDR);
    }
}

fn kvmmake(pgtbl: &mut PageTable) {
    // uart registers
    kvmmap(pgtbl, UART, UART, PGSIZE, PTE_R | PTE_W);

    // virtio mmio disk interface
    //kvmmap(kpgtbl, VIRTIO0, VIRTIO0, PGSIZE, PTE_R | PTE_W);
    // not implmented yet

    // PLIC
    //kvmmap(pgtbl, PLIC, PLIC, 0x400000, PTE_R | PTE_W);
    // not implmented yet

    // map kernel text executable and read-only.
    kvmmap(
        pgtbl,
        KERNELBASE,
        KERNELBASE,
        get_etext() - KERNELBASE,
        PTE_R | PTE_X,
    );

    kvmmap(
        pgtbl,
        get_etext(),
        get_etext(),
        PHYSTOP - get_etext(),
        PTE_R | PTE_W,
    );
    // map the trampoline for trap entry/exit to
    // the highest virtual address in the kernel.
    kvmmap(pgtbl, TRAMPOLINE, get_trampoline(), PGSIZE, PTE_R | PTE_X)
}

fn kvmmap(pgtbl: &mut PageTable, va: usize, pa: usize, sz: usize, perm: u64) {
    if !mappages(pgtbl, va, pa, sz, perm) {
        panic!("kvmmap error!");
    }
}

// Create PTEs for virtual addresses starting at va that refer to
// physical addresses starting at pa. va and size might not
// be page-aligned. Returns true on success, false if walk() couldn't
// allocate a needed page-table page.
fn mappages(pgtbl: &mut PageTable, va: usize, pa: usize, sz: usize, perm: u64) -> bool {
    if sz == 0 {
        panic!("mappages: size of zero");
    }

    let mut a = PGROUNDDOWN!(va);
    let last = PGROUNDDOWN!(va + sz - 1);
    let mut pa = pa;
    loop {
        let pte = walk(pgtbl, a, true).expect("mappages: walk return error!");
        if (*pte & PTE_V) == 1 {
            panic!("mappages: remap");
        }
        *pte = PA2PTE!(pa as u64) | perm | PTE_V;
        if a == last {
            break;
        }
        a += PGSIZE;
        pa += PGSIZE;
    }
    true
}

fn walk(pgtbl: &mut PageTable, va: usize, alloc: bool) -> Result<&mut u64, ()> {
    let mut pgtb_addr: *mut [u64; 512] = &mut pgtbl.ptes as *mut [u64; 512]; // turn a around with rust's safety requirement
    if va >= MAXVA as usize {
        panic!("walk: virtual address excess MAXVA");
    }

    for level in (1..=2).rev() {
        let pte: &mut u64 = unsafe { &mut (*pgtb_addr)[PX!(level, va)] };
        if (*pte & PTE_V) == 1 {
            pgtb_addr = PTE2PA!(*pte) as *mut [u64; 512];
        } else {
            if !alloc {
                return Err(());
            }
            pgtb_addr = kalloc() as *mut [u64; 512];
            unsafe {
                (*(pgtb_addr as *mut [u64; 512])).as_mut_slice().fill(0);
            }
            *pte = PA2PTE!(pgtb_addr as u64) | PTE_V;
        }
    }
    Ok(unsafe { &mut (*pgtb_addr)[PX!(0, va)] })
}

fn kalloc() -> *mut u8 {
    unsafe {
        ALLOCATOR
            .lock()
            .allocate_first_fit(Layout::from_size_align_unchecked(PGSIZE, PGSIZE))
            .expect("kalloc error")
            .as_ptr()
    }
}

pub fn kvminithart() {
    w_satp(MAKE_SATP!(unsafe { KERN_PG_ADDR }));
    sfence_vma();
}
