use std::process::{Command, ExitStatus};

#[derive(Debug)]
pub struct Reset;

impl Reset {
    pub fn clear() -> Option<ExitStatus> {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .ok()
    }
}
