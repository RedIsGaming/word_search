use rand::{distributions::Alphanumeric, thread_rng, Rng};
use crate::puzzle::Puzzle;

#[derive(Debug)]
pub struct Grid;

impl Grid {
    pub fn board<T>(word: &T, puzzle: &Puzzle)
    where
        String: for<'a> From<&'a T>,
    {   
        let words = String::from(word);
        let mut bytes = words
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        let byte: Vec<_> = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(puzzle.width as usize - bytes.len())
            .map(char::from)
            .collect();

        bytes.extend(byte
            .into_iter()
            .map(|x| x.to_string().to_uppercase())
        );

        if thread_rng().gen() {
            bytes.reverse();
        }

        println!("{}", bytes.join(" ").to_uppercase());
    }
}
