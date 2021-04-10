use rand::prelude::*;

use super::{Generator, SquareGrid};
use crate::grids::core::Direction;

/// Randomly choose to move either east or north. If east isn't possible, go north.
/// If north isn't possible, go east. If neither are possible, do nothing.
pub struct BinaryTree;

impl<G: SquareGrid> Generator<G> for BinaryTree {
    fn gen(grid: &mut G) {
        let mut rng = rand::thread_rng();

        for i in 0..grid.cells().len() {
            let cell = &grid.cells()[i];

            let mut choices = Vec::new();

            if cell.neighbours.contains(&Direction::E) {
                choices.push(Direction::E);
            }

            if cell.neighbours.contains(&Direction::N) {
                choices.push(Direction::N);
            }

            if let Some(dir) = choices.choose(&mut rng) {
                grid.link(i, *dir);
            }
        }
    }
}
