use crate::{
    memolayout::{plic_sclaim, plic_senable, plic_spriority, PLIC, UART_IRQ, VIRTIO0_IRQ},
    proc::cpuid,
};

pub fn plicinit() {
    let virtio_irq_ptr = (PLIC + VIRTIO0_IRQ * 4) as *mut u32;
    let uart_irq = (PLIC + UART_IRQ * 4) as *mut u32;
    unsafe {
        (*uart_irq) = 1;
        (*virtio_irq_ptr) = 1;
    }
}

pub fn plicinithart() {
    let hart = cpuid();
    let senable_addr = plic_senable(hart) as *mut u32;
    let spriority = plic_spriority(hart) as *mut u32;
    unsafe {
        *senable_addr = (1 << VIRTIO0_IRQ) | (1 << UART_IRQ);
        *spriority = 0;
    }
}

pub fn plic_claim() -> u32 {
    let hart = cpuid();
    let irq_addr = plic_sclaim(hart) as *const u32;
    unsafe { *irq_addr }
}

pub fn plic_complete(irq: u32) {
    let hart = cpuid();
    let irq_addr = plic_sclaim(hart) as *mut u32;
    unsafe {
        *irq_addr = irq;
    }
}
