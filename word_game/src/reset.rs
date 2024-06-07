use std::process::{Command, ExitStatus};

pub fn clear() -> Option<ExitStatus> {
    Command::new("cmd").args(["/c", "cls"]).status().ok()
}
