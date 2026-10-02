use std::process::exit;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("[+] Hyperlite test daemon launch...");
    sleep(Duration::new(3, 0));
    println!("[+] Hyperlite test daemon spindown!");
    exit(0);
}