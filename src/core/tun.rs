use libc::{c_int, c_short, close, ifreq, ioctl, open, read, write};
use std::{
    ffi::{CString, c_void},
    os::unix::io::{AsRawFd, RawFd},
};

const IFF_TUN: c_short = 0x0001;
const IFF_NO_PI: c_short = 0x1000;
const TUNSETIFF: c_int = 0x400454d2; // macOS-specific value (differs from Linux)

#[cfg(target_os = "macos")]
pub struct Tun {
    pub name: String,
    pub fd: RawFd,
}

#[cfg(target_os = "macos")]
impl Tun {
    pub fn new(name: String) -> std::io::Result<Self> {
        // Dynamically find available utunX
        let utun_path = find_available_utun()?;
        let fd = unsafe { open(utun_path.as_ptr() as *const i8, libc::O_RDWR) };
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

        if unsafe { ioctl(fd, TUNSETIFF as _, &ifr) } < 0 {
            let err = std::io::Error::last_os_error();
            unsafe { close(fd) };
            return Err(err);
        }

        Ok(Tun { fd, name })
    }

    pub fn read<'a>(&self, buf: &'a mut [u8]) -> std::io::Result<&'a [u8]> {
        let n = unsafe { read(self.fd, buf.as_mut_ptr() as _, buf.len()) };
        if n < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(&buf[..n as usize])
    }

    pub fn write(&self, data: &[u8]) -> std::io::Result<()> {
        let n = unsafe { write(self.fd, data.as_ptr() as _, data.len()) };
        if n < 0 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(())
    }
}

#[cfg(target_os = "macos")]
fn find_available_utun() -> std::io::Result<CString> {
    for i in 0..16 {
        // Check utun0 to utun15
        let path = format!("/dev/utun{}", i);
        let c_path = CString::new(path)?;
        let fd = unsafe { open(c_path.as_ptr() as *const i8, libc::O_RDWR) };
        if fd >= 0 {
            unsafe { close(fd) }; // Close and reuse
            return Ok(c_path);
        }
    }
    Err(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "No available utun device",
    ))
}

#[cfg(target_os = "macos")]
impl Drop for Tun {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}

// For cross-platform: Your original Linux code can be wrapped in #[cfg(target_os = "linux")]

