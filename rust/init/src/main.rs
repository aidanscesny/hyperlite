mod filesystem;
use rustix::{fs, system};

const CLEAR_TERM: &str = "\x1b[2J\x1b[H";

fn main() {
	println!("{}", CLEAR_TERM);
	println!("####################");
	println!("# Hyperlite v{} #", env!("CARGO_PKG_VERSION"));
	println!("####################");

	early_boot();
	flex_on_user();
	spawn_child();
	wait_for_user();
	graceful_shutdown();
}

fn wait_for_user() {
	println!("[?] Press Enter to power-off.");
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("[!] Failed to read input");
}

fn early_boot() {
	match filesystem::mount_fs(c"proc", c"/proc") {
		Ok(()) => {
			println!("[+] proc mounted at /proc");
		}

		Err(error) => {
			println!("[!] Failed to mount /proc");
			println!("[!] Error: {}", error.to_string());
			panic!();
		}
	}

	match filesystem::mount_fs(c"sysfs", c"/sys") {
		Ok(()) => {
			println!("[+] sysfs mounted at /sys");
		}

		Err(error) => {
			println!("[!] Failed to mount /sys");
			println!("[!] Error: {}", error.to_string());
			println!("[?] Press Enter to power-off.");
			let mut input = String::new();
			std::io::stdin()
				.read_line(&mut input)
				.expect("[!] Failed to read input");
			graceful_shutdown();
		}
	}

	match filesystem::mount_fs(c"devtmpfs", c"/dev") {
		Ok(()) => {
			println!("[+] devtmpfs mounted at /dev");
		}

		Err(error) => {
			println!("[!] Failed to mount /dev");
			println!("[!] Error: {}", error.to_string());
			println!("[?] Press Enter to power-off.");
			let mut input = String::new();
			std::io::stdin()
				.read_line(&mut input)
				.expect("[!] Failed to read input");
			graceful_shutdown();
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

	match filesystem::get_str_from_fd(&proc_fd) {
		Ok(str) => {
			println!("[+] Kernel Version: {}", str);
		}

		Err(error) => {
			println!("[-] Failed to parse kernel version: {}", error.to_string())
		}
	}

	// Print out dev
	match filesystem::print_dir("/dev") {
		Ok(()) => {}

		Err(error) => {
			println!("[!] Failed to parse /dev: {}", error.to_string())
		}
	}
}

fn spawn_child() {
	match std::process::Command::new("/system/bin/hyperlite-testd").spawn()
	{
		Ok(mut res) => {
			println!("[+] Spawned child: {}", res.id());
			loop {
				match res.try_wait() {
					Ok(None) => {
						std::thread::sleep(std::time::Duration::new(1, 0));
					}

					Ok(Some(status)) => {
						println!("[+] Child exited status {}", status.to_string());
						break;
					}

					Err(error) => {
						println!("[!] Error with child: {}", error.to_string());
						break;
					}
				}
			}
		}

		Err(error) => {
			println!("[!] Failed to spawn child: {}", error.to_string())
		}
	}
}

fn graceful_shutdown() {
	println!("[+] Shutting down...");

	system::reboot(system::RebootCommand::PowerOff).
		expect("Expected kernel to accept power-off command");
}