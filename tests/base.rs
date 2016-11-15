extern crate libc;
#[macro_use(ioctl)]
extern crate nix;

use std::mem::transmute_copy;

extern crate ifcfg;

use ifcfg::ffi;

#[test]
fn test_if_index_to_name() {
    let mut ifni = ifcfg::ffi::ifreq_index{
        ifr_name : [0u8; ifcfg::ffi::IFNAMSIZ],
        ifr_index : 2,
    };
    ioctl!(read get_if_name with 0x12, 0u8, ifcfg::ffi::ifreq_index);
}