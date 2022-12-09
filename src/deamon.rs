use std::process::{Command , Stdio , Child}
use std::io::{Read , Write}

pub fn init_deamon() {
    let mut child = Command::new("target/master")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    stdin.write_all(b")
}