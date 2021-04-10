use rand::prelude::*;

use super::{Generator, SquareGrid};

pub struct Backtracker;

impl<G: SquareGrid> Generator<G> for Backtracker {
    fn gen(grid: &mut G) {
        let mut rng = rand::thread_rng();

        // Add a random cell to the stack as the starting cell.
        let mut stack = vec![rng.gen_range(0..grid.cells().len())];

        while !stack.is_empty() {
            let idx = *stack.last().unwrap();
            let nbs = grid.unlinked_neighbours(idx);

            // Choose a random direction in which an unlinked neighbour lies.
            match nbs.choose(&mut rng) {
                // Link to the random neighbour if it exists.
                Some(&dir) => {
                    grid.link(idx, dir);
                    stack.push(grid.neighbour_index(idx, dir).unwrap());
                }

                // Backtrack if the random neighbour doesn't exist.
                None => {
                    stack.pop();
                }
            }
        }
    }
}
