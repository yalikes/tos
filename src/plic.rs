use crate::{memolayout::{VIRTIO0_IRQ, PLIC, plic_senable, plic_spriority, plic_sclaim}, proc::cpuid};

pub fn plicinit(){
    let virtio_irq_ptr = (PLIC + VIRTIO0_IRQ*4) as *mut u32;
    unsafe{
        (*virtio_irq_ptr) = 1;
    }
}

pub fn plicinithart(){
    let hart = cpuid();
    let senable_addr=plic_senable(hart) as *mut u32;
    let spriority = plic_spriority(hart) as *mut u32;
    unsafe{
        *senable_addr = 1 << VIRTIO0_IRQ;
        *spriority = 0;
    }
}

pub fn plic_claim() -> u32{
    let hart = cpuid();
    let irq_addr = plic_sclaim(hart) as *const u32;
    unsafe{
        *irq_addr
    }
}

pub fn plic_complete(irq: u32){
    let hart = cpuid();
    let irq_addr = plic_sclaim(hart) as *mut u32;
    unsafe{
        *irq_addr = irq;
    }
}