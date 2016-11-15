use libc;
use nix;

pub const IFNAMSIZ: usize = 16;
pub const SIOCGIFNAME: usize = 0x8910;       /* get iface name               */

#[repr(C)]
#[derive(Debug)]
pub struct sockaddr {
    sa_family: u16,
    sa_data: [u8; 14],
}

#[repr(C)]
#[derive(Debug)]
pub struct ifmap {
    mem_start: libc::c_uint,
    mem_end: libc::c_uint,
    base_addr: libc::c_ushort,
    irq: libc::c_uchar,
    dma: libc::c_uchar,
    port: libc::c_uchar,
}

#[repr(C)]
#[derive(Debug)]
pub struct ifreq_index {
    pub ifr_name: [u8; IFNAMSIZ],
    pub ifr_ifindex: libc::c_int,
}

#[repr(C)]
#[derive(Debug)]
pub struct ifreq {
    ifr_name: [u8; IFNAMSIZ], // Interface name
    ifr_data: [u8; IFNAMSIZ], /* union {
                               *     ifr_addr : sockaddr,
                               *     ifr_dstaddr : sockaddr,
                               *     ifr_broadaddr : sockaddr,
                               *     ifr_netmask : sockaddr,
                               *     ifr_hwaddr : sockaddr,
                               *     ifr_flags : libc::c_short,
                               *     ifr_ifindex : libc::c_int,
                               *     ifr_metric: libc::c_int,
                               *     ifr_mtu : libc::c_int,
                               *     ifmap   : ifr_map : ifmap,
                               *     ifr_slave: [u8, IFNAMSIZ],
                               *     ifr_newname:[ u8,IFNAMSIZ],
                               *     ifr_data: *libc::c_void,
                               * }, */
}

#[repr(C)]
#[derive(Debug)]
struct ifconf {
    ifc_len: libc::c_int, // size of buffer
    ifc_buf_or_req: *mut libc::c_void, /* union {
                                        *     char           *ifc_buf, /* buffer address */
                                        *     struct ifreq   *ifc_req, /* array of structures */
                                        * }, */
}