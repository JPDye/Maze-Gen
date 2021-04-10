#[allow(unused_imports)]
use image::imageops::{resize, FilterType};

use std::time::Instant;

use maze_gen::grids::animated_grid::AnimatedGrid;
use maze_gen::grids::core::{Animated, SquareGrid};
use maze_gen::grids::grid::Grid;

use maze_gen::generation::{
    AldousBroder, Backtracker, BinaryTree, Division, Ellers, Generator, HuntAndKill, Sidewinder,
    Wilsons,
};

fn main() {
    let start = Instant::now();

    let path = "./images/test.gif";

    let mut grid = AnimatedGrid::new(25, 25);
    Backtracker::gen(&mut grid);

    grid.animate(path);

    println!("Completed in {:?}", start.elapsed());
}
