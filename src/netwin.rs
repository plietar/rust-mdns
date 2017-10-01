extern crate winapi;
extern crate kernel32;
extern crate get_if_addrs;

use std::io;
use std::net::{IpAddr};

pub fn gethostname() -> io::Result<String> {
    const MAX_COMPUTERNAME_LENGTH: usize = 15;

    let mut buf = [0 as winapi::CHAR; MAX_COMPUTERNAME_LENGTH + 1];
    let mut len = buf.len() as u32;

    unsafe {
        if kernel32::GetComputerNameA(buf.as_mut_ptr(), &mut len) == 0 {
            return Err(io::Error::last_os_error());
        };
    }

    let host: Vec<u8> = buf[0..len as usize]
                            .iter()
                            .map(|&e| e as u8)
                            .collect();

    Ok(String::from_utf8_lossy(&host).into_owned())
}

pub struct InterfaceAddress {
    iface: get_if_addrs::Interface
}

impl InterfaceAddress {
    pub fn is_loopback(&self) -> bool {
        self.iface.is_loopback()
    }

    pub fn ip(&self) -> Option<IpAddr> {
       Some(self.iface.ip())
    }
}

pub fn getifaddrs() -> Vec<InterfaceAddress> {
    let mut ret = Vec::<InterfaceAddress>::new();

    for iface in get_if_addrs::get_if_addrs().unwrap() {
        ret.push(InterfaceAddress {
            iface: iface,
        });
    }

    return ret;
}
