use std;
use libc;
use ffi;

pub fn get_ipv4byname(name: &str) -> std::io::Result<std::net::Ipv4Addr> {
    let mut ifni = ffi::ifreq_addr::new();
    ifni.ifr_name.set(name);
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP);
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let rt: libc::c_int = libc::ioctl(fd, ffi::SIOCGIFADDR, &mut ifni);
        if rt != 0 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        libc::close(fd);
        return Ok(std::net::Ipv4Addr::from(ifni.ifr_addr.get_addr()));
    }
}

pub fn get_maskv4byname(name: &str) -> std::io::Result<std::net::Ipv4Addr> {
    let mut ifni = ffi::ifreq_addr::new();
    ifni.ifr_name.set(name);
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP);
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let rt: libc::c_int = libc::ioctl(fd, ffi::SIOCGIFNETMASK, &mut ifni);
        if rt != 0 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        libc::close(fd);
        return Ok(std::net::Ipv4Addr::from(ifni.ifr_addr.get_addr()));
    }
}

pub fn set_ipv4byname(name: &str, ip: std::net::Ipv4Addr) -> std::io::Result<()> {
    let mut ifni = ffi::ifreq_addr::new();
    ifni.ifr_name.set(name);
    ifni.ifr_addr.set_addr(ip);
    ifni.ifr_addr.set_family(super::AF_INET);
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP);
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let rt: libc::c_int = libc::ioctl(fd, ffi::SIOCSIFADDR, &mut ifni);
        if rt != 0 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        libc::close(fd);
    }
    return Ok(());
}

pub fn set_maskv4byname(name: &str, ip: std::net::Ipv4Addr) -> std::io::Result<()> {
    let mut ifni = ffi::ifreq_addr::new();
    ifni.ifr_name.set(name);
    ifni.ifr_addr.set_addr(ip);
    ifni.ifr_addr.set_family(super::AF_INET);
    unsafe {
        let fd: libc::c_int = libc::socket(libc::AF_INET, libc::SOCK_DGRAM, libc::IPPROTO_IP);
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let rt: libc::c_int = libc::ioctl(fd, ffi::SIOCSIFNETMASK, &mut ifni);
        if rt != 0 {
            libc::close(fd);
            return Err(std::io::Error::last_os_error());
        }
        libc::close(fd);
    }
    return Ok(());
}
