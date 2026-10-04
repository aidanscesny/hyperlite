mod filesystem;
mod console;

use rustix::{fs, system};
use std::sync::mpsc;
use std::time::Duration;

#[derive(Eq, PartialEq)]
pub enum ConsoleMsg {
	RequestShutdown
}

fn main() {
	println!("# Hyperlite v{} #", env!("CARGO_PKG_VERSION"));
	early_boot();
	flex_on_user();
	// spawn_child();

	let (console_tx,console_rx) =
		mpsc::channel::<ConsoleMsg>();
	let console_handler = console::init(console_tx);
	main_loop(console_rx);
	console_handler.join().expect("Thread should rejoin before shutdown");
	graceful_shutdown();
}

fn main_loop(recv: mpsc::Receiver<ConsoleMsg>){
	loop {
		match recv.try_recv() {
			Ok(msg) => {
				if msg == ConsoleMsg::RequestShutdown {
					return
				}
			}

			Err(mpsc::TryRecvError::Empty) => {
				std::thread::sleep(Duration::from_secs(1));
			}

			Err(mpsc::TryRecvError::Disconnected) => {
				println!("[!] Console disconnected... powering off");
				return
			}
		}
	}
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