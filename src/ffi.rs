use libc;
use std::net::Ipv4Addr;

pub const IFNAMSIZ: usize = 16;
pub const SIOCGIFNAME: libc::c_ulong = 0x8910;       /* get iface name               */
pub const SIOCGIFADDR: libc::c_ulong = 0x8915;       /* get PA address               */
pub const SIOCSIFADDR: libc::c_ulong = 0x8916;       /* set PA address               */
pub const SIOCGIFNETMASK: libc::c_ulong = 0x891b;    /* get network PA mask          */
pub const SIOCSIFNETMASK: libc::c_ulong = 0x891c;    /* set network PA mask          */

#[repr(C)]
#[derive(Debug)]
pub struct sockaddr {
    pub sa_family: u16,
    pub sa_data: [u8; 14],
}

impl sockaddr {
    pub fn new() -> sockaddr {
        sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        }
    }

    pub fn get_family(&self) -> u16 {
        self.sa_family
    }

    pub fn set_family(&mut self, fa: u16) {
        self.sa_family = fa;
    }

    pub fn get_port(&self) -> u16 {
        (self.sa_data[0] as u16) | ((self.sa_data[1] as u16) << 8)
    }

    pub fn set_port(&mut self, port: u16) {
        self.sa_data[0] = port as u8;
        self.sa_data[1] = (port >> 8) as u8;
    }

    pub fn get_addr(&self) -> Ipv4Addr {
        Ipv4Addr::new(self.sa_data[2],
                      self.sa_data[3],
                      self.sa_data[4],
                      self.sa_data[5])
    }

    pub fn set_addr(&mut self, addr: Ipv4Addr) {
        let arr = addr.octets();
        self.sa_data[2] = arr[0];
        self.sa_data[3] = arr[1];
        self.sa_data[4] = arr[2];
        self.sa_data[5] = arr[3];
    }
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
pub struct ifr_name_t {
    pub ifr_name: [u8; IFNAMSIZ],
}

impl ifr_name_t {
    pub fn new() -> ifr_name_t {
        ifr_name_t { ifr_name: [0; IFNAMSIZ] }
    }

    pub fn set(&mut self, s: &str) {
        let len = if s.as_bytes().len() > IFNAMSIZ {
            IFNAMSIZ
        } else {
            s.as_bytes().len()
        };
        for i in 0..len {
            self.ifr_name[i] = s.as_bytes()[i];
        }
    }

    pub fn to_string(&self) -> String {
        String::from_utf8_lossy(&self.ifr_name[..]).to_string()
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ifreq_index {
    pub ifr_name: ifr_name_t,
    pub ifr_ifindex: libc::c_int,
}

impl ifreq_index {
    pub fn new() -> ifreq_index {
        ifreq_index {
            ifr_name: ifr_name_t::new(),
            ifr_ifindex: 0,
        }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ifreq_addr {
    pub ifr_name: ifr_name_t,
    pub ifr_addr: sockaddr,
}

impl ifreq_addr {
    pub fn new() -> ifreq_addr {
        ifreq_addr {
            ifr_name: ifr_name_t::new(),
            ifr_addr: sockaddr::new(),
        }
    }
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

// #[repr(C)]
// #[derive(Debug)]
// struct ifconf {
//     ifc_len: libc::c_int, // size of buffer
//     ifc_buf_or_req: *mut libc::c_void, /* union {
//                                         *     char           *ifc_buf, /* buffer address */
//                                         *     struct ifreq   *ifc_req, /* array of structures */
//                                         * }, */
// }