use std::{io::{stdin, stdout, Write}, ops::Not};

use colored::Colorize;
use crate::{puzzle::PuzzleFile, parse::Parse, reset::Reset};

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

#[derive(Debug, Default)]
pub struct Field;

impl Field {
    pub fn generate() {
        Reset::clear();
        let mut input = String::new();

        println!("{}\n{}", 
            "Choose a appropriate field size between 5-20 for Reddy word_search:".bold().underline(), 
            "Default is 5 if this condition is not set properly.".red()
        );

        stdin().read_line(&mut input).ok();
        stdout().lock().flush().ok();
        
        Reset::clear();
        println!("{}", "Reddy word_search puzzle grid is:".bold().underline());
        
        let output = Parse::convert::<u16>(&input).unwrap_or_default();
        let size = Field::insert::<u16>(output);
        PuzzleFile::read(size)
    }
    
    fn insert<T: FieldRange + Default>(output: T) -> T {
        if output.fieldrange(START, END).not() {
            PuzzleFile::read(START);
            return T::default()
        }
        
        output
    }
}
