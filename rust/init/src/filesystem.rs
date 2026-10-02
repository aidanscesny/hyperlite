use std::error::Error;
use std::ffi::CStr;
use std::os::fd::OwnedFd;
use rustix::{io, mount};
use rustix::path::Arg;

pub fn mount_fs(src: &CStr, target: &CStr) -> Result<(), Box<dyn Error>> {
    Ok(mount::mount(
        src,
        target,
        src,
        mount::MountFlags::empty(),
        c""
    )?)
}

pub fn get_str_from_fd(fd: &OwnedFd) -> Result<String, Box<dyn Error>> {
    let mut buf = [0_u8; 64];
    let mut vec = Vec::<u8>::new();

    loop {
        let bytes_read = io::read(fd, &mut buf)?;
        vec.extend_from_slice(&buf[..bytes_read]);
        if bytes_read == 0 { break; }
    }

    Ok(String::from_utf8(vec)?)
}

pub fn print_dir(dir: &str) -> Result<(), Box<dyn Error>>{
    for entry in std::fs::read_dir(dir)? {
        println!("[+] Discovered: {}", entry?.path().to_string_lossy());
    }

    Ok(())
}