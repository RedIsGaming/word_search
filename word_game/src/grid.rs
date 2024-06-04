use std::{sync::Arc, fmt};

#[derive(Debug)]
pub struct Grid;

impl Grid {
    pub fn print<T>(word: &T) 
        where T: fmt::Debug,
    {
        let vec_2d: Vec<Vec<&T>> = vec![vec![word]];
    
        vec_2d.iter().for_each(|x| {
            println!("{:?}", x);
        });

        Grid::board(vec_2d);
    }

    fn board<T>(vec_2d: Vec<Vec<&T>>) 
        where T: fmt::Debug,
    {
        let bytes: Arc<Vec<Vec<&T>>> = Arc::new(vec_2d);
        println!("Board letters {:?}", bytes);
    }
}
