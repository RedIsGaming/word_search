use std::fmt;
use std::io::stdin;
use std::sync::Arc;

use crate::game::Wordgame;
use crate::parse::Parse;

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
            1 => Difficulty::field(input),
            2 => Difficulty::field(input),
            3 => Difficulty::field(input),
            _ => {
                input.clear();
                Wordgame::reset_option();
                Difficulty::print();

                stdin().read_line(&mut input).unwrap();
                let option = Parse::new(&input);
                Difficulty::new(option, input);
            },
        }
    }

    pub fn print() {
        println!("{}\n{}\n{}", Difficulty::Small, Difficulty::Normal, Difficulty::Large)
    }

    fn field(input: String) {
        let size = Parse::new(&input);
        let arc: Arc<Vec<Vec<u8>>> = Arc::new(vec![vec![1; (size * 6).into()]; (size * 6).into()].into());

        arc.iter().for_each(|vec| {
            println!("{:?}", vec);
        })
    }
}
