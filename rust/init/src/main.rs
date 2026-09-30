use std::error::Error;
use std::os::fd::OwnedFd;
use rustix::{fs, io, mount, system};

const CLEAR_TERM: &str = "\x1b[2J\x1b[H";

fn main() {
	println!("{}", CLEAR_TERM);
	println!("####################");
	println!("# Hyperlite v0.0.2 #");
	println!("####################");

	mount_proc();
	flex_on_user();

	println!("[?] Press Enter to power-off.");
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("[!] Failed to read input");

	graceful_shutdown();
}

fn mount_proc() {
	let proc_mount_flags:mount::MountFlags = mount::MountFlags::empty();

	mount::mount(c"proc", c"/proc",c"proc",proc_mount_flags,c"").
		expect("Kernel should mount proc at boot");

	println!("[+] Processes mounted at /proc");
}

fn get_str_from_fd(fd: &OwnedFd) -> Result<String, Box<dyn Error>> {
	let mut buf = [0_u8; 64];
	let mut vec = Vec::<u8>::new();

	loop {
		let result = io::read(fd, &mut buf);

		match result {
			Ok(bytes_read) => {
				vec.extend_from_slice(&buf[..bytes_read]);
				if bytes_read == 0 { break; }
			}

			Err(error) => {
				return Err(Box::from(error))
			}
		}
	}

	let ret = String::from_utf8(vec);

	match ret {
		Ok(str) => {
			Ok(str)
		}

		Err(error) => {
			Err(Box::from(error))
		}
	}
}

fn flex_on_user() {
	println!();
	println!("[+] PID: {}", std::process::id());

	// Print kernel version
	let proc_fd =
		fs::open("/proc/version", fs::OFlags::empty(), fs::Mode::empty()).
			expect("Expected valid file descriptor for /proc/version");

	match get_str_from_fd(&proc_fd) {
		Ok(str) => {
			println!("[+] Kernel Version: {}", str);
		}

		Err(error) => {

		}
	}

	// Print memory info
	let proc_fd =
		fs::open("/proc/meminfo", fs::OFlags::empty(), fs::Mode::empty()).
			expect("Expected valid file descriptor for /proc/meminfo");

	match get_str_from_fd(&proc_fd) {
		Ok(str) => {
			println!("[+] Kernel Memory: {}", str);
		}

		Err(error) => {

		}
	}
}

fn graceful_shutdown() {
	println!("[+] Shutting down gracefully.");

	system::reboot(system::RebootCommand::PowerOff).
		expect("Expected kernel to accept power-off command");
}