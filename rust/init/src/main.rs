use rustix::{fs, io, mount, system};

fn main() {
	println!("Hyperlite v0.0.1");

	mount_proc();
	flex_on_user();

	println!("Press Enter to power-off.");
	let mut input = String::new();
	std::io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");

	graceful_shutdown();
}

fn mount_proc() {
	let proc_mount_flags:mount::MountFlags = mount::MountFlags::empty();

	mount::mount(c"proc", c"/proc",c"proc",proc_mount_flags,c"").
		expect("Kernel should mount proc at boot");

	println!("Processes mounted at /proc");
}

fn flex_on_user() {
	println!();
	println!("Welcome to userspace");
	println!("PID: {}", std::process::id());

	// Print kernel version
	let proc_fd =
		fs::open("/proc/version", fs::OFlags::empty(), fs::Mode::empty()).
			expect("Expected valid file descriptor for /proc/version");

	let mut proc_version_buf = [0_u8; 64];
	io::read(proc_fd, &mut proc_version_buf).
		expect("Expected to read /proc/version");

	let proc_version_str = String::from_utf8_lossy(&proc_version_buf);

	println!("Kernel Version: {}", proc_version_str);

	// Print memory info
	let proc_fd =
		fs::open("/proc/meminfo", fs::OFlags::empty(), fs::Mode::empty()).
			expect("Expected valid file descriptor for /proc/meminfo");

	let mut proc_info_buf = [0_u8; 1024];
	io::read(proc_fd, &mut proc_info_buf).
		expect("Expected valid file descriptor for /proc/meminfo");

	let proc_info_str = String::from_utf8_lossy(&proc_info_buf);

	println!("Kernel Memory: {}", proc_info_str);
}

fn graceful_shutdown() {
	println!("Shutting down gracefully.");

	let cmd_reboot : system::RebootCommand = system::RebootCommand::PowerOff;
	system::reboot(cmd_reboot).
		expect("Expected kernel to accept power-off command");
}