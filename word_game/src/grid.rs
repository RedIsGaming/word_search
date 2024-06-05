use std::{cmp::Ordering, fmt, ops::{Add, Index, IndexMut, Mul}};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid;

impl Grid {
    pub fn board<T>(word: &T, puzzle: &Puzzle) 
    where 
        T: fmt::Debug,
        String: for<'a> From<&'a T>,
    {
        let words = String::from(word);
        let bytes = words.chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        
        let mut vec_2d: Grid2d<String> = Grid2d::new(puzzle.width, puzzle.height);
        vec_2d.grid.push(bytes);
        vec_2d.grid.sort_by(|a, b| b.cmp(a).partial_cmp(&Ordering::Equal).unwrap());
        
        for byte in vec_2d.grid {
            println!("{}", byte) //join can be used to prevent std::fmt::Display from popping up.
        }
    }
}

#[derive(Debug, Clone)]
struct Grid2d<T> {
    grid: Vec<T>,
    x: u16,
    y: u16,
}

impl<T> Grid2d<T> {
    fn new(x: u16, y: u16) -> Self {
        Self {
           grid: Vec::with_capacity(x.into()),
           x,
           y,
        }
    }

    fn collect_index(&self, index: Idx) -> Option<usize> {
        let (x, y) = index;
        let mut z = None;

        if x.lt(&self.x.into()) && y.lt(&self.y.into()) {
            z = Some(x.mul(y).add(x));
            return z;
        }

        z
    }
}

type Idx = (usize, usize);

impl<T> Index<Idx> for Grid2d<T> {
    type Output = T;

    fn index(&self, index: Idx) -> &Self::Output {
        let collect_index = self.collect_index(index);
        &self.grid[collect_index.unwrap_or_default()]
    }
}

impl<T> IndexMut<Idx> for Grid2d<T> {
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        let collect_index = self.collect_index(index);
        &mut self.grid[collect_index.unwrap_or_default()]
    }
}
