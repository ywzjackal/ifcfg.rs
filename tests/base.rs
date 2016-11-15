extern crate libc;
use std::net::Ipv4Addr;

extern crate ifcfg;

#[test]
fn test_get_ipv4byname() {
    assert_eq!(ifcfg::get_ipv4byname("lo").unwrap(),
               Ipv4Addr::new(127, 0, 0, 1));
}