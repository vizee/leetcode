use std::process;
use std::thread;
use std::time::Duration;

#[macro_use]
mod leetcode;
mod solution;

const MAX_EXEC_SECS: u64 = 3;

fn main() {
    thread::spawn(|| {
        thread::sleep(Duration::from_secs(MAX_EXEC_SECS));
        eprintln!("out of time");
        process::exit(1);
    });
    println!("=========================");
    solution::run();
    println!("=========================");
}
