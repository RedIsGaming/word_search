use std::{fmt, process};
use std::io::{stdout, stdin, Write};
use webbrowser;

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
    pub fn print() {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Prank, Menu::Quit)
    }

    pub fn menu(option: u8, input: String) {
        match option {
            1 => Menu::play(),
            2 => Menu::prank(),
            3 => Menu::quit(),
            _ => Menu::default(input),
        }
    }

    fn play() {
        Wordgame::clear();
        println!("You won!!!");

        let input = String::new();
        Menu::repeat(input);
    }

    fn prank() {
        println!("Hehehehehe");
        webbrowser::open("https://www.youtube.com/watch?v=cvh0nX08nRw").unwrap();
    }

    fn quit() {
        println!("Goodbye!!!");
        process::exit(0)
    }

    fn default(mut input: String) {
        input.clear();
        Wordgame::clear();
        Menu::repeat(input);
    }

    fn repeat(mut input: String) {
        stdout().lock().flush().unwrap();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        Menu::print();

        stdin().read_line(&mut input).unwrap();
        let option = Parse::parse(&input);
        Menu::menu(option, input);
    }
}
