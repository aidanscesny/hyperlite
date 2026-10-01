mod filesystem;
use rustix::{fs, system};

const CLEAR_TERM: &str = "\x1b[2J\x1b[H";

fn main() {
	println!("{}", CLEAR_TERM);
	println!("####################");
	println!("# Hyperlite v0.0.2 #");
	println!("####################");

	filesystem::mount_proc();
	println!("[+] Proc mounted at /proc");

	filesystem::mount_sysfs();
	println!("[+] Sysfs mounted at /sys");

	flex_on_user();

	println!("[?] Press Enter to power-off.");
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("[!] Failed to read input");

	graceful_shutdown();
}

fn flex_on_user() {
	println!();
	println!("[+] PID: {}", std::process::id());

	// Print kernel version
	let proc_fd =
		fs::open("/proc/version", fs::OFlags::empty(), fs::Mode::empty()).
			expect("Expected valid file descriptor for /proc/version");

	match filesystem::get_str_from_fd(&proc_fd) {
		Ok(str) => {
			println!("[+] Kernel Version: {}", str);
		}

		Err(error) => {
			println!("[-] Failed to parse kernel version: {}", error.to_string())
		}
	}
}

fn graceful_shutdown() {
	println!("[+] Shutting down gracefully.");

	system::reboot(system::RebootCommand::PowerOff).
		expect("Expected kernel to accept power-off command");
}