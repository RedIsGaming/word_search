use std::{fmt, process};
use std::process::{Command, ExitStatus};
use std::str::FromStr;
use std::error;
use std::io::{stdout, stdin, Write};

#[derive(Debug)]
enum Menu {
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
    fn play() -> () {
        println!("Play!")
    }

    fn prank() -> () {
        println!("Prank")
    }

    fn quit() -> () {
        println!("Goodbye!!!");
        process::exit(0)
    }
}

#[derive(Debug)]
struct Wordgame;

impl Wordgame {
    fn clear() -> ExitStatus {
        Command::new("cmd")
            .args(&["/c", "cls"])
            .status()
            .unwrap()
    }

    fn new() -> Result<(), Box<dyn error::Error>> {
        Wordgame::clear();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        let mut input = String::new();
        
        stdout().lock().flush()?;
        Wordgame::print_menu();
        stdin().read_line(&mut input)?;

        let option = Field::parse(&input);
        Wordgame::menu(option);
        Ok(())
    }

    fn print_menu() -> () {
        println!("{}\n{}\n{}", Menu::Wordgame, Menu::Prank, Menu::Quit)
    }

    fn menu(option: u8) -> () {
        match option {
            1 => Menu::play(),
            2 => Menu::prank(),
            3 => Menu::quit(),
            _ => panic!("Invalid option chosen..."),
        }
    }
}

#[derive(Debug)]
struct Field;

impl Field {
    fn parse(s: &str) -> u8 {
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

fn main() -> Result<(), Box<dyn error::Error>> {
    let game = Wordgame::new();
    game
}
