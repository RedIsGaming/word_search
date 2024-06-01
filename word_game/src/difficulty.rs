use std::fmt;
use std::io::{stdin, stdout, Write};

use crate::game::Wordgame;
use crate::parse::Parse;
use crate::field::Field;

#[derive(Debug)]
pub enum Difficulty {
    Small,
    Normal,
    Large,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Small => write!(f, "1. Small - (6x6)"),
            Difficulty::Normal => write!(f, "2. Normal - (12x12)"),
            Difficulty::Large => write!(f, "3. Large - (18x18)"),
        }
    }
}

impl Difficulty {
    pub fn new(option: u8, mut input: String) {
        match option {
            1 => Field::new(input),
            2 => Field::new(input),
            3 => Field::new(input),
            _ => {
                Wordgame::reset_option(&mut input);
                Difficulty::print();
                stdout().write_all(b"Enter a whole number between 0-255!\n").unwrap();

                stdin().read_line(&mut input).unwrap();
                let option = Parse::new(&input).expect("Expected a whole number between 0-255");
                Difficulty::new(option, input);
            },
        }
    }

    pub fn print() {
        println!("{}\n{}\n{}", Difficulty::Small, Difficulty::Normal, Difficulty::Large)
    }
}
