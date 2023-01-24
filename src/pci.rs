use crate::{println, vm::kalloc};

#[derive(Debug)]
#[repr(C)]
struct PCIConfigurationSpcaeHeader{
    vendor_id: u16,
    device_id: u16,
    command: u16,
    status: u16,
    revision_id: u8,
    class_code: [u8;3],
    cache_line_size: u8,
    master_latency_time: u8,
    header_type: u8,
    built_in_self_test: u8,
    remain_part: [u8; 240],
}

#[repr(C)]
struct PCIConfigurationSpcaeHeaderType0{
    base_address_registers: [u8; 24],
    cardbus_cis_pointer: u32,
    subsystem_vendor_id: u16,
    subsystem_id: u16,
    expansion_rom_base_address: u32,
    capabilities_pointer: u8,
    reserved: [u8; 7],
    interrupt_line: u8,
    interrupt_pin: u8,
    min_gnt: u8,
    max_lat: u8,
}

pub fn list_pci(addr: usize){
    unsafe{
        let csh: &PCIConfigurationSpcaeHeader = &*(addr as *const PCIConfigurationSpcaeHeader);
        //println!("{:?}", csh); this cause run out of stack
        println!("device id: {:04x}:{:04x}", csh.vendor_id, csh.device_id);
        println!("header type: {}", csh.header_type);
        println!("command: {:x}", csh.command);
        if csh.header_type != 0{
            return
        }
        let type0_header: &mut PCIConfigurationSpcaeHeaderType0 = &mut *(&csh.remain_part as *const u8 as u64 as *mut PCIConfigurationSpcaeHeaderType0);
        println!("subsystem id: {:04x}:{:04x}", type0_header.subsystem_vendor_id, type0_header.subsystem_id);
        // type0_header.expansion_rom_base_address = 0xff_ff_ff_ff;
        // println!("exp: {:x}", type0_header.expansion_rom_base_address);
    }
}

pub unsafe fn write_vga(addr: usize){
    let csh: &mut PCIConfigurationSpcaeHeader = &mut *(addr as *mut PCIConfigurationSpcaeHeader);
    csh.command = csh.command | 0b10;
    println!("Command: {}", csh.command);
    if csh.header_type != 0{
        return
    }
    let type0_header: &PCIConfigurationSpcaeHeaderType0 = &*((&csh.remain_part as *const u8) as u64 as *const PCIConfigurationSpcaeHeaderType0);
    let bar_0: &mut u32 = &mut *(&type0_header.base_address_registers as *const u8 as u64 as *mut u32);
    let bar_2: &u32 = &*((&type0_header.base_address_registers as *const u8 as u64 + 32*2) as *const u32);
    let bar_6: &u32 = &*((&type0_header.base_address_registers as *const u8 as u64 + 32*2) as *const u32);
    let bar_0_val =0x8100_0000;
    *bar_0 = bar_0_val;
    println!("{:x}", *bar_0);
    println!("{:x}", bar_0_val);
}