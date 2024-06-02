use std::{error, process, process::*, io::{stdout, stdin, Write}};
use webbrowser;

use crate::difficulty::Difficulty;
use crate::menu::Menu;
use crate::parse::Parse;

#[derive(Debug)]
pub struct Wordgame;

impl Wordgame {
    pub fn build() -> Result<(), Box<dyn error::Error>> {
        Wordgame::clear();
        
        let mut input = String::new();
        stdout().write_all(b"Determine an option for Reddy Wordgame:\n")?;
        Menu::print();

        stdin().read_line(&mut input)?;
        let option = Parse::convert::<u8>(&input).unwrap_or_default();
        Menu::create(option.into(), input);
        
        Ok(())
    }

    pub fn clear() -> Option<ExitStatus> {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .ok()
    }

    pub fn play(mut old_input: String) {
        Wordgame::reset_option(&mut old_input);
        drop(old_input);
        
        let mut new_input = String::new();
        Difficulty::print();

        stdin().read_line(&mut new_input).ok();
        let option = Parse::convert::<u8>(&new_input).unwrap_or_default();
        Difficulty::select(option.into(), new_input);
    }

    pub fn prank() {
        webbrowser::open("https://www.youtube.com/watch?v=cvh0nX08nRw").ok();
    }

    pub fn quit() {
        process::exit(0)
    }

    pub fn reset_option(input: &mut String) {
        input.clear();
        Wordgame::clear();
        stdout().write_all(b"Determine an option for Reddy Wordgame:\n").ok();
        stdout().lock().flush().ok();
    }
}
