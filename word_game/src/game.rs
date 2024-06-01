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
        Wordgame::clear();
        
        let mut input = String::new();
        stdout().write_all(b"Determine an option for Reddy Wordgame:\n")?;
        Menu::print();

        stdin().read_line(&mut input)?;
        let option = Parse::new(&input)?;
        Menu::new(option, input);
        
        Ok(())
    }

    pub fn clear() -> ExitStatus {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap()
    }

    pub fn play(mut old_input: String) {
        Wordgame::reset_option(&mut old_input);
        drop(old_input);
        
        let mut new_input = String::new();
        Difficulty::print();

        stdin().read_line(&mut new_input).unwrap();
        let option = Parse::new(&new_input).unwrap();
        Difficulty::new(option, new_input);
    }

    pub fn prank() {
        webbrowser::open("https://www.youtube.com/watch?v=cvh0nX08nRw").unwrap();
    }

    pub fn quit() {
        process::exit(0)
    }

    pub fn reset_option(input: &mut String) {
        input.clear();
        Wordgame::clear();
        stdout().write_all(b"Determine an option for Reddy Wordgame:\n").unwrap();
        stdout().lock().flush().unwrap();
    }
}
