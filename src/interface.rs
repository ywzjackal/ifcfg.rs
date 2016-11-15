use std::net;

#[derive(PartialEq, Eq, Debug)]
pub enum Kind {
    Packet,
    Ipv4,
    Ipv6,
}

#[derive(PartialEq, Eq, Debug)]
pub enum NextHop {
    Broadcast(net::SocketAddr),
    Destination(net::SocketAddr),
}

#[derive(Debug)]
pub struct Interface {
    /// The name of this interface.
    pub name: String,

    /// The kind of interface this is.
    pub kind: Kind,

    /// The address of this interface, if it has one.
    pub addr: Option<net::SocketAddr>,

    /// The netmask of this interface, if it has one.
    pub mask: Option<net::SocketAddr>,

    /// The broadcast address or destination address, if it has one.
    pub hop: Option<NextHop>,
}
