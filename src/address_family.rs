use net2::UdpBuilder;
use net2::unix::UnixUdpBuilderExt;
use std::io;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, UdpSocket};
use super::MDNS_PORT;
use libc;

pub enum Inet {}
pub enum Inet6 {}

pub trait AddressFamily {
    fn bind() -> io::Result<UdpSocket> {
        let addr = SocketAddr::new(Self::any_addr(), MDNS_PORT);
        let builder = Self::socket_builder()?;
        builder.reuse_address(true)?;
        match builder.reuse_port(true) {
            Ok(_) => {},
            // On linux kernel < 3.9 reuse_port is not available. Ignore
            // the error on all linux kernel versions. If reuse_port is mandatory for some
            // reason the bind() call will fail later.
            #[cfg(target_os = "linux")]
            Err(ref e) if e.raw_os_error() == Some(libc::ENOPROTOOPT) => {},
            Err(err) => panic!("reuse_port failed: {}", err)
        }
        let socket = builder.bind(&addr)?;
        Self::join_multicast(&socket)?;
        Ok(socket)
    }

    fn socket_builder() -> io::Result<UdpBuilder>;
    fn any_addr() -> IpAddr;
    fn mdns_group() -> IpAddr;
    fn join_multicast(socket: &UdpSocket) -> io::Result<()>;
    fn v6() -> bool;
}

impl AddressFamily for Inet {
    fn socket_builder() -> io::Result<UdpBuilder> {
        UdpBuilder::new_v4()
    }
    fn any_addr() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(0,0,0,0))
    }
    fn mdns_group() -> IpAddr {
        IpAddr::V4(Ipv4Addr::new(224,0,0,251))
    }
    fn join_multicast(socket: &UdpSocket) -> io::Result<()> {
        socket.join_multicast_v4(
            &Ipv4Addr::new(224,0,0,251),
            &Ipv4Addr::new(0,0,0,0),
        )
    }
    fn v6() -> bool {
        false
    }
}

impl AddressFamily for Inet6 {
    fn socket_builder() -> io::Result<UdpBuilder> {
        UdpBuilder::new_v6()
    }
    fn any_addr() -> IpAddr {
        IpAddr::V6(Ipv6Addr::new(0,0,0,0,0,0,0,0))
    }
    fn mdns_group() -> IpAddr {
        IpAddr::V6(Ipv6Addr::new(0xff02,0,0,0,0,0,0,0xfb))
    }
    fn join_multicast(socket: &UdpSocket) -> io::Result<()> {
        socket.join_multicast_v6(
            &Ipv6Addr::new(0xff02,0,0,0,0,0,0,0xfb),
            0
        )
    }
    fn v6() -> bool {
        true
    }
}
