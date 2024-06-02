use std::fs::File;
use std::io::{self, BufReader, Read};

#[derive(Debug)]
pub struct WordFile;

impl WordFile {
    fn new() -> Result<File, io::Error> {
        match File::open("../public/word_search.txt") {
            Ok(file) => Ok(file),
            Err(err) => return Err(err),
        }
    }

    pub fn read() {
        let file = WordFile::new().expect("Couldn't open the words file");
        let mut bufreader = BufReader::new(file);
        let mut words = String::new();

        bufreader.read_to_string(&mut words).unwrap();
        print!("{}", words);
    }
}
