use std::io::prelude::*;
use std::process::{Command, Stdio};


pub fn execute_rofi(value: &str) -> String {
    let process = Command::new("rofi")
        .arg("-dmenu")
        .arg("-lines")
        .arg("5")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    process.stdin.unwrap().write_all(value.as_bytes()).unwrap();
    let mut s = String::new();
    process.stdout.unwrap().read_to_string(&mut s).unwrap();
    s
}
