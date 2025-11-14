use std::{
    ffi::CString,
    os::unix::io::{AsRawFd, RawFd},
};

const TUNSETIFF: c_int = 0x400454ca;
const IFF_TUN: c_short = 0x0001;
const IFF_TAP: c_short = 0x0002;
const IFF_NO_PI: c_short = 0x1000;

use libc::{c_int, c_short, ifreq};

pub struct Tun {
    name: String,
    fd: RawFd,
}

impl Tun {
    pub fn new(name: String) -> std::io::Result<Self> {
        let fd = unsafe { libc::open(b"/dev/net/tun\0".as_ptr() as _, libc::O_RDWR) };
        if fd < 0 {
            return Err(std::io::Error::last_os_error());
        }
        let mut ifr: ifreq = unsafe { std::mem::zeroed() };
        let name_cstr = CString::new(name.clone()).unwrap();
        unsafe {
            std::ptr::copy_nonoverlapping(
                name_cstr.as_ptr(),
                ifr.ifr_name.as_mut_ptr(),
                name.len(),
            );
        }
        ifr.ifr_ifru.ifru_flags = (IFF_TUN | IFF_NO_PI) as _;
        if unsafe { libc::ioctl(fd, TUNSETIFF as _, &ifr) } < 0 {
            let err = std::io::Error::last_os_error();
            unsafe { libc::close(fd) };
            return Err(err);
        }

        Ok(Tun {
            fd,
            name: name.to_string(),
        })
    }

    pub fn read<'a>(&self, buf: &'a mut [u8]) -> std::io::Result<&'a [u8]> {
        let n = unsafe { libc::read(self.fd, buf.as_mut_ptr() as _, buf.len()) };
        if n < 0 {
            return Err(std::io::Error::last_os_error());
        } else {
            return Ok(&buf[..n as usize]);
        }
    }

    pub fn write(&self, data: &[u8]) -> std::io::Result<()> {
        let n = unsafe { libc::write(self.fd, data.as_ptr() as _, data.len()) };
        if n < 0 {
            return Err(std::io::Error::last_os_error());
        } else {
            Ok(())
        }
    }
}
