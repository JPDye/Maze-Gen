use image::RgbaImage;

use std::collections::HashSet;
use std::iter::FromIterator;

pub trait Animated: SquareGrid {
    fn animate(&self, path: &str);
}

pub trait SquareGrid {
    fn rows(&self) -> usize;
    fn cols(&self) -> usize;

    fn cells(&self) -> &Vec<Cell>;

    fn neighbour(&self, idx: usize, dir: Direction) -> Option<&Cell>;
    fn neighbour_index(&self, idx: usize, dir: Direction) -> Option<usize>;

    fn linked_neighbours(&self, idx: usize) -> Vec<Direction>;
    fn unlinked_neighbours(&self, idx: usize) -> Vec<Direction>;

    fn link(&mut self, idx: usize, dir: Direction);

    fn save(&self) -> RgbaImage;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Clone, Debug, Default)]
pub struct Cell {
    pub neighbours: HashSet<Direction>,
    pub links: HashSet<Direction>,
}

impl From<Vec<Direction>> for Cell {
    fn from(dirs: Vec<Direction>) -> Self {
        Self {
            neighbours: HashSet::from_iter(dirs.into_iter()),
            links: HashSet::new(),
        }
    }
}
