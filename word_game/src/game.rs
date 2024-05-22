use std::error;
use std::io::{stdout, stdin, Write};
use std::process::{Command, ExitStatus};

use crate::menu::Menu;
use crate::parse::Parse;

#[derive(Debug)]
pub struct Wordgame;

impl Wordgame {
    pub fn clear() -> ExitStatus {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap()
    }

    pub fn new() -> Result<(), Box<dyn error::Error>> {
        Wordgame::clear();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        let mut input = String::new();
        
        stdout().lock().flush()?;
        Menu::print();
        stdin().read_line(&mut input)?;

        let option = Parse::parse(&input);
        Menu::menu(option, input);
        Ok(())
    }
}
