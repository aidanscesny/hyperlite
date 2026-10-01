use std::error::Error;
use std::os::fd::OwnedFd;
use rustix::{io, mount};

pub fn mount_proc() {
    let mount_flags = mount::MountFlags::empty();

    mount::mount(c"proc", c"/proc",c"proc",mount_flags,c"").
        expect("Kernel should mount proc at boot");
}

pub fn mount_sysfs() {
    let mount_flags = mount::MountFlags::empty();

    mount::mount(c"sysfs", c"/sys",c"sysfs",mount_flags,c"").
        expect("Kernel should mount sysfs at boot");
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