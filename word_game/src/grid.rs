use std::{fmt, ops::{Add, Index, IndexMut, Mul}};

use crate::puzzle::Puzzle;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid;

impl Grid {
    pub fn board<T>(word: &T, puzzle: &Puzzle) 
        where 
            T: fmt::Debug,
            String: for<'a> From<&'a T>,
    {
        let byte = String::from(word);
        let mut vec_2d = Grid2d::new(puzzle.width, puzzle.height);
        vec_2d.grid.push(byte.chars());

        println!("{:?}", vec_2d.grid);
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
           grid: Vec::with_capacity(x.mul(y).into()),
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
