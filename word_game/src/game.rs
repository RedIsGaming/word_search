use std::error;
use std::io::{stdout, stdin, Write};
use crate::menu::Menu;
use crate::field::Field;

#[derive(Debug)]
pub struct Wordgame;

impl Wordgame {
    pub fn new() -> Result<(), Box<dyn error::Error>> {
        Menu::clear();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        let mut input = String::new();
        
        stdout().lock().flush()?;
        Menu::new();
        stdin().read_line(&mut input)?;

        let option = Field::parse(&input);
        Wordgame::menu(option, input);
        Ok(())
    }

    pub fn play() -> () {
        Menu::clear();
        let _ = stdout().write_all(b"Determine an option:\n");
        let mut input = String::new();
        
        stdout().lock().flush().unwrap();
        Menu::won();
        stdin().read_line(&mut input).unwrap();

        let option = Field::parse(&input);
        Wordgame::wordgame_menu(option, input);
    }

    pub fn menu(option: u8, input: String) -> () {
        match option {
            1 => Wordgame::play(),
            2 => Menu::prank(),
            3 => Menu::quit(),
            _ => Wordgame::default(input),
        }
    }
    fn wordgame_menu(option: u8, input: String) -> () {
        match option {
            1 => Wordgame::play(),
            2 => Menu::back(),
            3 => Menu::quit(),
            _ => Wordgame::wordgame_default(input),
        }
    }

    fn default(mut input: String) -> () {
        input.clear();

        Menu::clear();
        let _ = stdout().write_all(b"Determine an option for Reddy Wordgame:\n");
        Menu::new();

        stdin().read_line(&mut input).unwrap();
        let option = Field::parse(&input);
        Wordgame::menu(option, input);
    }

    fn wordgame_default(mut input: String) -> () {
        input.clear();

        Menu::clear();
        let _ = stdout().write_all(b"Determine an option:\n");
        Menu::won();
        
        stdin().read_line(&mut input).unwrap();
        let option = Field::parse(&input);
        Wordgame::wordgame_menu(option, input)
    }
}
