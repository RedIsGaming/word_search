use std::process::{Command, ExitStatus};
use std::error;
use std::io::{stdout, stdin, Write};
use word_game::*;

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
        Menu::print_menu();
        stdin().read_line(&mut input)?;

        let option = Field::parse(&input);
        Wordgame::menu(option, input);
        Ok(())
    }

    fn menu(option: u8, input: String) -> () {
        match option {
            1 => Menu::play(),
            2 => Menu::prank(),
            3 => Menu::quit(),
            _ => Wordgame::default(input),
        }
    }

    fn default(mut input: String) -> () {
        input.clear();

        Wordgame::clear();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        Menu::print_menu();

        stdin().read_line(&mut input).unwrap();
        let option = Field::parse(&input);
        Wordgame::menu(option, input);
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let game = Wordgame::new();
    game
}
