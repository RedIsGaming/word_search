use std::{fmt, process};
use std::process::{Command, ExitStatus};
use std::io::{stdout, stdin, Write};
use crate::field::Field;
use crate::game::Wordgame;

#[derive(Debug)]
pub enum Menu {
    Wordgame,
    Prank,
    Quit,
    Back,
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {        
        match self {
            Menu::Wordgame => write!(f, "1. Wordgame"),
            Menu::Prank => write!(f, "2. Prank"),
            Menu::Quit => write!(f, "3. Quit"),
            Menu::Back => write!(f, "2. Back"),
        }
    }
}

impl Menu {
    pub fn clear() -> ExitStatus {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap()
    }

    pub fn prank() -> () {
        println!("Prank")
    }

    pub fn quit() -> () {
        println!("Goodbye!!!");
        process::exit(0)
    }

    pub fn new() -> () {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Prank, Menu::Quit)
    }

    pub fn back() -> () {
        Menu::clear();
        let _ = stdout().write_all(b"Determine an option:\n");
        let mut input = String::new();

        stdout().lock().flush().unwrap();
        Menu::new();
        stdin().read_line(&mut input).unwrap();

        let option = Field::parse(&input);
        Wordgame::menu(option, input);
    }

    pub fn won() -> () {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Back, Menu::Quit)
    }
}
