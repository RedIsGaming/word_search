use std::{
    io::{stdin, stdout, Write},
    ops::Not,
};

use crate::{parse, puzzle, reset};
use colored::Colorize;

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

pub fn generate() {
    reset::clear();
    let mut input = String::new();

    println!(
        "{}\n{}",
        "Choose a appropriate field size between 5-20 for Reddy word_search:".bold().underline(),
        "Default is 5 if this condition is not set properly.".red()
    );

    stdin().read_line(&mut input).ok();
    stdout().lock().flush().ok();

    reset::clear();
    println!("{}", "Reddy word_search puzzle grid is:".bold().underline());

    let output = parse::convert::<u16>(&input).unwrap_or_default();
    let size = insert::<u16>(output);
    puzzle::read(size)
}

fn insert<T: FieldRange + Default>(output: T) -> T {
    if output.fieldrange(START, END).not() {
        puzzle::read(START);
        return T::default();
    }

    output
}
