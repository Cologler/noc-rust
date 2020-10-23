#![windows_subsystem = "windows"]

use std::process::Command;
use std::os::windows::process::CommandExt;

fn main() {
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    if std::env::args().count() > 1 {
        let cmd = std::env::args().nth(1).unwrap();
        let mut proc = Command::new(cmd)
            .args(std::env::args().skip(2))
            .creation_flags(CREATE_NO_WINDOW)
            .spawn()
            .expect("failed to execute");
        proc.wait().expect("who care");
    }
}
