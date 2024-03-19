use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let output = Command::new("echo")
        .args(&args[1..])
        .output()
        .expect("Failed to execute command");

    println!("{}", String::from_utf8_lossy(&output.stdout));
}