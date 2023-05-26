pub struct virtio_pci_cap{
    cap_vndr: u8,  /* Generic PCI field: PCI_CAP_ID_VNDR */
    cap_next: u8,
    cap_len: u8,
    cfg_type: u8,
    bar: u8,
    padding: [u8; 3],
    offset: u32,
    length: u32,
}