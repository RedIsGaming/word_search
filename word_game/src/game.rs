use std::{error, process};
use std::io::{stdout, stdin, Write};
use std::process::{Command, ExitStatus};
use webbrowser;

use crate::difficulty::Difficulty;
use crate::menu::Menu;
use crate::parse::Parse;

#[derive(Debug)]
pub struct Wordgame;

impl Wordgame {
    pub fn new() -> Result<(), Box<dyn error::Error>> {
        Wordgame::reset_option();
        let mut input = String::new();
        Menu::print();

        stdin().read_line(&mut input)?;
        let option = Parse::new(&input);
        Menu::new(option, input);
        
        Ok(())
    }

    pub fn clear() -> ExitStatus {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap()
    }

    pub fn play() {
        Wordgame::reset_option();
        let mut input = String::new();
        Difficulty::print();

        stdin().read_line(&mut input).unwrap();
        let option = Parse::new(&input);
        Difficulty::new(option, input);
    }

    pub fn prank() {
        webbrowser::open("https://www.youtube.com/watch?v=cvh0nX08nRw").unwrap();
    }

    pub fn quit() {
        process::exit(0)
    }

    pub fn reset_option() {
        Wordgame::clear();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        stdout().lock().flush().unwrap();
    }
}
