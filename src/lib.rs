extern crate libc;

pub mod ffi;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod linux;

pub use linux::*;
