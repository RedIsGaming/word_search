use std::{fmt, io::{stdin, stdout, Write}, process::{Command, ExitStatus}};

use crate::puzzle::PuzzleFile;
use crate::parse::Parse;

pub trait FieldRange {
    fn fieldrange(&self, start: u8, end: u8) -> bool;
}

impl FieldRange for u8 {
    fn fieldrange(&self, start: u8, end: u8) -> bool {
        *self >= start && *self <= end
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Field {
    size: u8,
}

impl Field {
    pub fn new(size: u8) -> Self {
        Self {
            size,
        }
    }

    pub fn clear() -> Option<ExitStatus> {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .ok()
    }

    pub fn generate() {
        let input = String::new();
        let output = Field::print(input);

        Field::insert::<u8>(output);
        PuzzleFile::read();
    }
    
    fn insert<T>(output: T) -> Field
        where 
            T: TryFrom<T> + Default + fmt::Display + FieldRange, 
            u8: TryInto<T> + From<T>,
    {
        if !output.fieldrange(5, 20) {
            let input = Field::print(output.to_string());
            Field::insert::<u8>(input);
            T::default();
        }

        let input = output.into();
        Field::new(input)
    }

    fn print(mut input: String) -> u8 {
        input.clear();
        Field::clear();

        stdout().lock().flush().unwrap();
        println!("Choose a appropriate field size between 5-20 for Reddy word_search:");
        stdin().read_line(&mut input).unwrap();

        Parse::convert::<u8>(&input).unwrap_or_default()
    }
}
