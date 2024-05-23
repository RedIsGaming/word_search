use std::fmt;
use std::io::stdin;

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
    pub fn new(option: u8, mut input: String) {
        match option {
            1 => Wordgame::play(),
            2 => Wordgame::prank(),
            3 => Wordgame::quit(),
            _ => {
                input.clear();
                Wordgame::reset_option();
                Menu::print();

                stdin().read_line(&mut input).unwrap();
                let option = Parse::new(&input);
                Menu::new(option, input);
            },
        }
    }

    pub fn print() {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Prank, Menu::Quit)
    }
}
