extern crate libc;
use std::net::Ipv4Addr;

extern crate ifcfg;

#[test]
fn test_get_ipv4byname() {
    let mut ifni = ifcfg::ffi::ifreq_addr::new();
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP);
        assert!(fd >= 0);
        ifni.ifr_name.set("lo");
        let rt: libc::c_int = libc::ioctl(fd, ifcfg::ffi::SIOCGIFADDR, &mut ifni);
        assert_eq!(rt, 0);
        assert_eq!(ifni.ifr_addr.get_family(), 2);
        assert_eq!(ifni.ifr_addr.get_port(), 0);
        assert_eq!(ifni.ifr_addr.get_addr(),
                   (127 << 0) | (0 << 8) | (0 << 16) | (1 << 24));
    }

    assert_eq!(ifcfg::get_ipv4byname("lo").unwrap(), Ipv4Addr::from((127 << 0) | (0 << 8) | (0 << 16) | (1 << 24)));
}