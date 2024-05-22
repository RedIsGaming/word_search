use std::{fmt, process};
use std::str::FromStr;

#[derive(Debug)]
pub enum Menu {
    Wordgame,
    Prank,
    Quit
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        match self {
            Menu::Wordgame => write!(f, "1. Wordgame"),
            Menu::Prank => write!(f, "2. Prank"),
            Menu::Quit => write!(f, "3. Quit"),
        }
    }
}

impl Menu {
    pub fn play() -> () {
        println!("Play!")
    }

    pub fn prank() -> () {
        println!("Prank")
    }

    pub fn quit() -> () {
        println!("Goodbye!!!");
        process::exit(0)
    }

    pub fn print_menu() -> () {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Prank, Menu::Quit)
    }
}

#[derive(Debug)]
pub struct Field;

impl Field {
    pub fn parse(s: &str) -> u8 {
        let parse = s.trim()
            .parse::<u8>()
            .map_err(|_| format!("Error while trying to parse to u8."))
            .expect("Expected a valid whole number between 0-255");

        let input = parse.to_string();

        match u8::from_str(&input) {
            Ok(num) => num,
            Err(_) => 0,
        }
    }
}
