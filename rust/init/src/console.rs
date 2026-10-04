use std::{sync, thread};
use crate::ConsoleMsg;

const CLEAR_TERM: &str = "\x1b[2J\x1b[H";

#[derive(Copy, Clone, Debug)]
pub enum LogLevel {
    Info,   // Green  [ + ]
    Warn,   // Yellow [ ~ ]
    Error   // Red    [ ! ]
}

impl LogLevel {
    fn prefix(self) -> &'static str {
        match self {
            LogLevel::Info =>  "\x1b[32m[ + ]\x1b[0m",
            LogLevel::Warn =>  "\x1b[33m[ ~ ]\x1b[0m",
            LogLevel::Error => "\x1b[31m[ ! ]\x1b[0m",
        }
    }
}

pub fn init(sender: sync::mpsc::Sender<ConsoleMsg>) -> thread::JoinHandle<()> {
    thread::spawn( move || {
        console_loop(sender);
    })
}

fn console_loop(sender: sync::mpsc::Sender<ConsoleMsg>) {
    clear_term();
    init_splash();

    println!("Press RETURN to power off");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("[!] Failed to read input");

    match sender.send(ConsoleMsg::RequestShutdown) {
        Ok(()) => {

        }

        Err(error) => {
            println!("[!] Fatal error: {}", error.to_string())
        }
    }
}

fn init_splash() {
    println!("####################");
    println!("# Hyperlite v{} #", env!("CARGO_PKG_VERSION"));
    println!("####################");
}

fn clear_term() {
    println!("{}", CLEAR_TERM);
}