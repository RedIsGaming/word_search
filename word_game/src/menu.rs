use std::{fmt, io::{stdin, stdout, Write}};

use crate::game::Wordgame;
use crate::parse::Parse;

#[derive(Debug)]
pub enum Menu {
    Wordgame,
    Prank,
    Quit,
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
    pub fn create(option: usize, mut input: String) {
        match option {
            1 => Wordgame::play(input),
            2 => Wordgame::prank(),
            3 => Wordgame::quit(),
            _ => {
                Wordgame::reset_option(&mut input);
                Menu::print();
                stdout().write_all(b"Enter a whole number between 0-255!\n").ok();

                stdin().read_line(&mut input).ok();
                let option = Parse::convert::<u8>(&input).unwrap_or_default();
                Menu::create(option.into(), input);
            },
        }
    }

    pub fn print() {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Prank, Menu::Quit)
    }
}
