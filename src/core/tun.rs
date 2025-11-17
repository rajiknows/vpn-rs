use libc::{
    AF_SYS_CONTROL, AF_SYSTEM, CTLIOCGINFO, SOCK_DGRAM, c_char, c_int, close, connect, getsockopt,
    read, socket, write,
};
use std::{
    ffi::{CStr, c_void},
    io, mem,
    os::unix::io::RawFd,
    ptr,
};

const SYSPROTO_CONTROL: c_int = 2;
const UTUN_CONTROL_NAME: &str = "com.apple.net.utun_control\0";
const UTUN_OPT_IFNAME: c_int = 2;

#[repr(C)]
struct ctl_info {
    ctl_id: u32,
    ctl_name: [c_char; 96],
}

#[repr(C)]
struct sockaddr_ctl {
    sc_len: u8,
    sc_family: u8,
    ss_sysaddr: u16,
    sc_id: u32,
    sc_unit: u32,
    sc_reserved: [u32; 5],
}

pub struct Tun {
    pub fd: RawFd,
    pub name: String,
}

impl Tun {
    pub fn new() -> io::Result<Self> {
        let fd = unsafe { socket(libc::PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }

        let mut info = ctl_info {
            ctl_id: 0,
            ctl_name: [0; 96],
        };
        unsafe {
            ptr::copy_nonoverlapping(
                UTUN_CONTROL_NAME.as_ptr() as *const c_char,
                info.ctl_name.as_mut_ptr(),
                UTUN_CONTROL_NAME.len() - 1,
            );
        }

        if unsafe { libc::ioctl(fd, CTLIOCGINFO, &mut info) } < 0 {
            let err = io::Error::last_os_error();
            unsafe { close(fd) };
            return Err(err);
        }

        let addr = sockaddr_ctl {
            sc_len: mem::size_of::<sockaddr_ctl>() as u8,
            sc_family: AF_SYSTEM as u8,
            ss_sysaddr: AF_SYS_CONTROL as u16,
            sc_id: info.ctl_id,
            sc_unit: 0,
            sc_reserved: [0; 5],
        };

        if unsafe {
            connect(
                fd,
                &addr as *const _ as *const libc::sockaddr,
                mem::size_of::<sockaddr_ctl>() as u32,
            )
        } < 0
        {
            let err = io::Error::last_os_error();
            unsafe { close(fd) };
            return Err(err);
        }

        let mut ifname = [0 as c_char; 32];
        let mut len = 32 as libc::socklen_t;
        if unsafe {
            getsockopt(
                fd,
                SYSPROTO_CONTROL,
                UTUN_OPT_IFNAME,
                ifname.as_mut_ptr() as *mut c_void,
                &mut len,
            )
        } < 0
        {
            let err = io::Error::last_os_error();
            unsafe { close(fd) };
            return Err(err);
        }

        let name = unsafe { CStr::from_ptr(ifname.as_ptr()) }
            .to_string_lossy()
            .into_owned();

        println!("Created utun interface: {}", name);

        Ok(Tun { fd, name })
    }

    pub fn read<'a>(&self, buf: &'a mut [u8]) -> io::Result<&'a [u8]> {
        let n = unsafe { read(self.fd, buf.as_mut_ptr() as *mut c_void, buf.len()) };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(&buf[..n as usize])
        }
    }

    pub fn write(&self, data: &[u8]) -> io::Result<()> {
        let n = unsafe { write(self.fd, data.as_ptr() as *const c_void, data.len()) };
        if n < 0 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

impl Drop for Tun {
    fn drop(&mut self) {
        unsafe { close(self.fd) };
    }
}
