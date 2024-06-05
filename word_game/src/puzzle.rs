use std::ops::{AddAssign, Not};
use std::{collections::HashSet, hash::Hash, hash};
use std::{fs::File, fmt};
use std::io::{self, BufReader, Read};

use rand::{thread_rng, seq::SliceRandom};
use crate::grid::Grid;

#[derive(Debug, Default)]
pub struct Puzzle {
    pub width: u16,
    pub height: u16,
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

    fn create(word: String, size: u16) -> Puzzle {
        let puzzle = Puzzle::new(size, size);
        let vec = Puzzle::iter(word, size);
        Puzzle::insert(puzzle, vec)
    }

    fn iter(word: String, size: u16) -> Vec<String> {
        let mut sum: usize = Default::default();
        let mut words: Vec<_> = word.split_whitespace()
            .map(|x| x.to_string())
            .collect();

        words.shuffle(&mut thread_rng());

        words.into_iter()
            .map(|x| x.to_string())
            .filter(|x| x.contains('-').not() && x.contains('(').not() && x.contains(')').not())
            .filter(|x| x.contains('\'').not() && x.contains('/').not() && x.contains('.').not())
            .filter(|x| x.len().eq(&size.into()))
            .take_while(|x| {
                sum.add_assign(x.as_bytes().len());
                sum.le(&size.pow(2).into())
            })
            .collect()
    }

    fn insert(puzzle: Puzzle, vec: Vec<String>) -> Puzzle {
        let mut binding: HashSet<_> = puzzle.word_search.iter().cloned().collect();

        for word in vec.iter() {
            binding.insert(word.to_string());
        }

        Puzzle::spawn::<String>(binding, puzzle)
    }

    fn spawn<T>(binding: HashSet<T>, puzzle: Puzzle) -> Puzzle 
    where 
        T: fmt::Debug,
        String: for<'a> From<&'a T>
    {
        for word in &binding {
            Grid::board(word, &puzzle);
        }

        Puzzle::new(Default::default(), Default::default())
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
        let mut word = String::new();

        bufreader.read_to_string(&mut word).ok();
        Puzzle::create(word, size);
    }
}
