use std::io::{stdin, stdout, Write};
use std::process::{Command, ExitStatus};

use crate::puzzle::PuzzleFile;
use crate::parse::Parse;

const START: u16 = 5;
const END: u16 = 20;

pub trait FieldRange {
    fn fieldrange(&self, start: u16, end: u16) -> bool;
}

impl FieldRange for u16 {
    fn fieldrange(&self, start: u16, end: u16) -> bool {
        self.ge(&start) && self.le(&end)
    }
}

#[derive(Debug)]
pub struct Field;

impl Field {
    pub fn clear() -> Option<ExitStatus> {
        Command::new("cmd")
            .args(["/c", "cls"])
            .status()
            .ok()
    }

    pub fn generate() {
        Field::clear();
        let mut input = String::new();

        println!(
            "Choose a appropriate field size between 5-20 for Reddy word_search:\nDefault is 5 if this condition is not set properly"
        );

        stdin().read_line(&mut input).ok();
        stdout().lock().flush().ok();
        
        let output = Parse::convert::<u16>(&input).unwrap_or_default();
        let size = Field::insert::<u16>(output);
        PuzzleFile::read(size)
    }
    
    fn insert<T>(output: T) -> T
        where 
            T: TryFrom<T> + FieldRange,
    {
        if !output.fieldrange(START, END) {
            PuzzleFile::read(START);
        }

        output
    }
}
