use std::io;

fn main() {
	println!("Hyperlite v0.0.1");
	println!();
	println!("Welcome to userspace");
	println!("PID: {}", std::process::id());
	println!("Press Enter to continue.");

	let mut input = String::new();
	io::stdin()
		.read_line(&mut input)
		.expect("Failed to read input");

	println!("Brace for panic.");
}
