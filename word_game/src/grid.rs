use std::{sync::Arc, fmt};

#[derive(Debug, Default)]
pub struct Grid;

impl Grid {
    pub fn new() -> Self {
        Self { }
    }

    pub fn print<T: fmt::Debug>(word: &T) -> Grid {
        let words = [word];
    
        words.iter().for_each(|x| {
            println!("Words: {:?}", x);
        });
        
        Grid::board::<&T>(words)
    }

    fn board<T: fmt::Debug>(words: [T; 1]) -> Grid {
        let vec_2d = vec![words];
        let bytes = Arc::new(vec_2d);
        println!("Bytes: {:?}", bytes);
        
        Grid::new()
    }
}
