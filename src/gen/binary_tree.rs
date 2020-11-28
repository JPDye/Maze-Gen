use rand::seq::SliceRandom;

use crate::cells::cell::Direction::*;
use crate::gen::generator::Generator;
use crate::grids::rect_grid::RectGrid;

pub struct BinaryTree {}

impl Generator for BinaryTree {
    fn gen(maze: &mut RectGrid) {
        for cell_rc in &maze.grid {
            let mut cell = cell_rc.borrow_mut();

            let mut possible = Vec::new();

            if cell.neighbours.get(&N).unwrap().is_some() {
                possible.push(N);
            }

            if cell.neighbours.get(&E).unwrap().is_some() {
                possible.push(E);
            }

            let choice = possible.choose(&mut rand::thread_rng());

            if let Some(&dir) = choice {
                cell.link(dir)
            }
        }
    }
}
