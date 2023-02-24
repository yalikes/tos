use core::mem::size_of;

use crate::println;
use crate::riscv::{PGSIZE, PGSHIFT};
use crate::virtio::virtio_blk::{ DiskPages, VirtqDesc, QUEUE_NUM, VirtqAvail, VirtqUsed};
use virtio_blk::DISK;

pub mod virtio_blk;



pub const MAGIC_VALUE: u32 = 0x74726976;
pub const DEVICE_VERSION: u32 = 0x1; //because QEMU use lagacy mmio interface

const STATUS_ACKNOWLEDGE: u32 = 1;
const STATUS_DRIVER: u32 = 2;
const STATUS_DRIVER_OK: u32 = 4;
const STATUS_FEATURES_OK: u32 = 8;
const STATUS_DEVICE_NEEDS_RESET: u32 = 64;
const STATUS_FAILED: u32 = 128;

const VIRTIO_F_INDIRECT_DESC: u32 = 1 << 28;
const VIRTIO_F_EVENT_IDX: u32 = 1 << 29;

#[repr(C, align(4096))]
struct MMIODeviceLagacyRegisterLayout {
    magic_value: u32,        //0x000
    version: u32,            //0x004
    device_id: u32,          //0x008
    vendor_id: u32,          //0X00c
    host_features: u32,      //0X010
    host_features_sel: u32,  //0x014,
    __padding_01: [u8; 8],   //0x018
    guest_features: u32,     //0x020
    guest_features_sel: u32, //0x024
    guest_page_size: u32,    //0x028
    __padding_02: [u8; 4],   //0x02c
    queue_sel: u32,          //0x030
    queue_num_max: u32,      //0x034
    queue_num: u32,          //0x038
    queue_align: u32,        //0x03c
    queue_phy_page_num: u32, //0x040
    __padding_03: [u8; 12],  //0x044
    queue_notify: [u32; 4],  //0x050
    interrupt_status: u32,   //0x060
    interrupt_ack: u32,      //0x064
    __padding_04: [u8; 8],   //0x068
    status: u32,             //0x070
    __pading_01: [u8; 140],  //0x074
    config: [u8; 0x100],     //0x100
}

pub fn check_virtio_device_is_valid(reg_addr: *const u8) -> bool {
    let dev_reg_ref = unsafe { &*(reg_addr as u64 as *const MMIODeviceLagacyRegisterLayout) };
    dev_reg_ref.magic_value == MAGIC_VALUE
        && dev_reg_ref.version == DEVICE_VERSION
        && dev_reg_ref.device_id != 0x0
}

pub fn init_virtio_device(dev_addr: *const u8) {
    let mut status;
    let dev_reg_ref = unsafe { &mut *(dev_addr as u64 as *mut MMIODeviceLagacyRegisterLayout) };
    if dev_reg_ref.device_id != virtio_blk::DEVICE_ID
        && dev_reg_ref.vendor_id != virtio_blk::VENDOR_ID
    {
        panic!("this device is not we want!");
    }

    status = 0;
    dev_reg_ref.status = status; //1. reset device
    status |= STATUS_ACKNOWLEDGE;
    dev_reg_ref.status = status; //2. set ACKNOWLEDGE bit
    status |= STATUS_DRIVER;
    dev_reg_ref.status = status; //3. set DRIVER bit

    let mut feature_bits: u32 = dev_reg_ref.host_features; //4. read features bit
    feature_bits &= !virtio_blk::VIRTIO_BLK_F_RO;
    feature_bits &= !virtio_blk::VIRTIO_BLK_F_SCSI;
    feature_bits &= !virtio_blk::VIRTIO_BLK_F_CONFIG_WCE;
    feature_bits &= !virtio_blk::VIRTIO_BLK_F_MQ;
    feature_bits &= !virtio_blk::VIRTIO_F_ANY_LAYOUT;
    feature_bits &= !VIRTIO_F_EVENT_IDX;
    feature_bits &= !VIRTIO_F_INDIRECT_DESC;

    dev_reg_ref.guest_features = feature_bits; //4. set features bit
    status |= STATUS_FEATURES_OK;
    dev_reg_ref.status = status; //5. set FEATURES_OK bit

    status = dev_reg_ref.status; // we have to use explicity write this line, so that compiler can generate 'lw' instruction, rather than 'lbu' instruction.

    if status & STATUS_FEATURES_OK == 0 {
        //6. check FEATURES_OK
        panic!("can't set FEATURES_OK");
    }

    status |= STATUS_DRIVER_OK;
    dev_reg_ref.status = status;

    dev_reg_ref.guest_page_size = PGSIZE as u32;
    dev_reg_ref.queue_sel = 0;
    let max = dev_reg_ref.queue_num_max;
    if max == 0 {
        panic!("virtio disk has no queue 0");
    }
    if max == virtio_blk::QUEUE_NUM as u32{
        panic!("virtio disk max queue too short");
    }
    dev_reg_ref.queue_num = virtio_blk::QUEUE_NUM as u32;
    dev_reg_ref.queue_phy_page_num = (&DISK.lock().pages as *const DiskPages as u64 >> PGSHIFT) as u32;

    let virtq_desc_addr = &DISK.lock().pages as *const DiskPages as usize + 0;// use +0 to prevent borrow behavior
    let virtq_avail_addr = &DISK.lock().pages as *const DiskPages as usize + QUEUE_NUM * size_of::<VirtqDesc>(); 
    let virtq_used_addr = &DISK.lock().pages as *const DiskPages as usize + PGSIZE;

    let mut disk_guard = DISK.lock();
    disk_guard.desc = virtq_desc_addr as *mut VirtqDesc;
    disk_guard.avail = virtq_avail_addr as *mut VirtqAvail;
    disk_guard.used = virtq_used_addr as *mut VirtqUsed;

    //unlike xv6, we do not need memset DISK.pages because we already do this in lazy_static,
    // the same for DISK.free
    println!("finish init blk dev");

}
