use libc;
use nix;

pub fn all() -> Result<Vec<String>, nix::Error> {
    nix::net::if_::if_nametoindex
    nix::sys::ioctl!();
}