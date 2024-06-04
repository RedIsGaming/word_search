use std::ops::{AddAssign, Not};
use std::{collections::HashSet, hash::Hash, hash};
use std::{fs::File, fmt};
use std::io::{self, BufReader, Read};

#[derive(Debug, Default)]
struct Puzzle {
    width: u16,
    height: u16,
    word_search: HashSet<String>,
}

impl Hash for Puzzle {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.width.hash(state);
        self.height.hash(state);
        self.word_search.hasher();
    }
    
    fn hash_slice<H: hash::Hasher>(data: &[Self], state: &mut H)
    where
        Self: Sized,
    {
        for piece in data {
            piece.hash(state)
        }
    }
}

impl Puzzle {
    fn new(width: u16, height: u16) -> Self {
        Self { 
            width, 
            height, 
            word_search: HashSet::new(), 
        }
    }

    fn create(words: String, size: u16) {
        let puzzle = Puzzle::new(size, size);
        let vec = Puzzle::iter(words, size);
        Puzzle::insert(puzzle, vec)
    }

    fn iter(words: String, size: u16) -> Vec<String> {
        let mut sum: u16 = Default::default();

        words.lines()
            .map(|x| x.to_string())
            .filter(|x| x.contains('-').not() && x.contains("()").not() && x.contains('\'').not())
            .filter(|x| x.len() <= size.into())
            .take_while(|x| {
                sum.add_assign(size);
                x.as_bytes().len().le(&sum.into())
            })
            .collect()
    }

    fn insert(puzzle: Puzzle, vec: Vec<String>) {
        let mut binding: HashSet<_> = puzzle.word_search.into_iter().collect();

        for word in vec.iter() {
            binding.insert(word.to_string());
        }

        Puzzle::print::<String>(binding);
    }

    fn print<T: fmt::Debug>(binding: HashSet<T>) {
        for word in &binding {
            println!("{:?}", word);
        }
    }
}

#[derive(Debug)]
pub struct PuzzleFile;

impl PuzzleFile {
    fn unlock() -> Result<File, io::Error> {
        match File::open("../public/word_search.txt") {
            Ok(file) => Ok(file),
            Err(err) => Err(err),
        }
    }

    pub fn read(size: u16) {
        let file = PuzzleFile::unlock().expect("Couldn't open the words file");
        let mut bufreader = BufReader::new(file);
        let mut words = String::new();

        bufreader.read_to_string(&mut words).ok();
        Puzzle::create(words, size);
    }
}
