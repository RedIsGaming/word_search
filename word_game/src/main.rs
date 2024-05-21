use std::process::{Command, ExitStatus};
use std::str::FromStr;
use std::error;
use std::io::{stdout, stdin, Write};

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

        let _ = stdout().write_all(b"Determine your word game board-size.\n");
        let mut input = String::new();
        stdout().lock().flush()?;

        stdin().read_line(&mut input)?;
        let x = Field::parse(&input);
        input.clear();

        stdin().read_line(&mut input)?;
        let y = Field::parse(&input);
        Field::new(x, y);

        Ok(())
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

    fn new(x: u8, y: u8) -> () {
        let mut vec: Vec<Vec<u8>> = vec![vec![1; y.into()]; x.into()];

        for x in vec.iter_mut() {
            for y in x.iter() {
                println!("The field to print: {:?}{}", x, y)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let game = Wordgame::new();
    game
}
