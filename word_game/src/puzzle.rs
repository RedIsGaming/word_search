use std::{collections::HashSet, hash::Hash, hash};
use std::{fs::File, io::{self, BufReader, Read}, fmt};

#[derive(Debug, Default)]
pub struct Puzzle {
    width: u8,
    height: u8,
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
    pub fn new(width: u8, height: u8) -> Self {
        Self { 
            width, 
            height, 
            word_search: HashSet::new(), 
        }
    }

    pub fn create(words: String) {
        let puzzle = Puzzle::new(5, 5);
        let replace = Puzzle::replace(words);
        Puzzle::insert(puzzle, replace)
    }

    fn replace(words: String) -> Vec<String> {
        words.lines()
            .map(|line| line.to_string())
            .collect()
    }

    fn insert(puzzle: Puzzle, mut replace: Vec<String>) {
        let mut binding: HashSet<_> = puzzle.word_search.into_iter()
            .map(&|words| words)
            .collect();

        replace.sort();

        for word in replace {
            binding.insert(word);
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

    pub fn read() {
        let file = PuzzleFile::unlock().expect("Couldn't open the words file");
        let mut bufreader = BufReader::new(file);
        let mut words = String::new();

        bufreader.read_to_string(&mut words).ok();
        Puzzle::create(words);
    }
}
