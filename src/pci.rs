use crate::{
    mem_utils,
    memolayout::{VGA_FRAME_BUFFER, VGA_FRAME_BUFFER_SIZE, VGA_MMIO_BASE},
    println,
    vm::kalloc,
};

#[derive(Debug)]
#[repr(C)]
struct PCIConfigurationSpcaeHeader {
    pub vendor_id: u16,
    pub device_id: u16,
    pub command: u16,
    pub status: u16,
    pub revision_id: u8,
    pub class_code: [u8; 3],
    pub cache_line_size: u8,
    pub master_latency_time: u8,
    pub header_type: u8,
    pub built_in_self_test: u8,
    pub remain_part: [u8; 240],
}

#[repr(C)]
pub struct PCIConfigurationSpcaeHeaderType0 {
    pub __padding: [u8; 16],
    pub base_address_registers: [u8; 24],
    pub cardbus_cis_pointer: u32,
    pub subsystem_vendor_id: u16,
    pub subsystem_id: u16,
    pub expansion_rom_base_address: u32,
    pub capabilities_pointer: u8,
    pub reserved: [u8; 7],
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
    pub min_gnt: u8,
    pub max_lat: u8,
}

pub fn list_pci(pci_base_addr: usize) {
    for dev_num in 0..(1 << 5) {
        for func_num in 0..(1 << 3) {
            //we list device in bus 0 only
            let config_space_addr = pci_base_addr + (dev_num << (12 + 3)) + (func_num << 12);
            let config_space_header =
                unsafe { &*(config_space_addr as *const PCIConfigurationSpcaeHeader) };
            if config_space_header.vendor_id == 0xffff {
                //ignore device with vendor id 0xffff
                continue;
            }
            println!(
                "vendor_id:device_id {:04x}:{:04x}",
                config_space_header.vendor_id, config_space_header.device_id
            );

            println!(
                "header_type {}, status: {}",
                config_space_header.header_type, config_space_header.status
            );
            if config_space_header.header_type == 0 {
                let type0_header = unsafe { &*(config_space_addr as *const PCIConfigurationSpcaeHeaderType0) };
                println!("capabilities pointer {:x}", type0_header.capabilities_pointer);
            }
            println!("----------------");
        }
    }
}

pub unsafe fn write_vga(addr: usize) {
    let csh: &mut PCIConfigurationSpcaeHeader = &mut *(addr as *mut PCIConfigurationSpcaeHeader);
    csh.command = csh.command | 0b10;
    println!("Command: {}", csh.command);
    if csh.header_type != 0 {
        return;
    }
    let type0_header: &PCIConfigurationSpcaeHeaderType0 =
        &*((&csh.remain_part as *const u8) as u64 as *const PCIConfigurationSpcaeHeaderType0);
    let bar_0: &mut u32 =
        &mut *(&type0_header.base_address_registers as *const u8 as u64 as *mut u32);
    let bar_2: &mut u32 =
        &mut *((&type0_header.base_address_registers as *const u8 as u64 + 4 * 2) as *mut u32);
    let bar_6: &mut u32 =
        &mut *((&type0_header.base_address_registers as *const u8 as u64 + 4 * 5) as *mut u32);
    let bar_0_val = VGA_FRAME_BUFFER;
    *bar_0 = bar_0_val as u32;
    *bar_2 = VGA_MMIO_BASE as u32;

    println!("{:x}", *bar_0);
    println!("{:x}", bar_0_val);
    let framebuffer: &mut [u8; VGA_FRAME_BUFFER_SIZE] =
        &mut *(bar_0_val as *mut [u8; VGA_FRAME_BUFFER_SIZE]);
    let vga_mmio: &mut [u8; 4096] = &mut *(VGA_MMIO_BASE as *mut [u8; 4096]);
    framebuffer.fill(0xff);
    vga_mmio[0] = 0x0c;
}
