// External imports
use rand::prelude::*;

// Self imports
use crate::{Generator, HardCellLink, RectGrid};

#[derive(Debug)]
pub struct Backtracker {}

impl Generator for Backtracker {
    fn gen(&self, maze: &mut RectGrid) {
        let mut rng = thread_rng();

        let mut stack: Vec<HardCellLink> = Vec::new();
        stack.push(maze.get_random_cell().unwrap());

        while !stack.is_empty() {
            let cell_rc = stack.last().unwrap();
            let mut cell = cell_rc.borrow_mut();

            // Pick a random unlinked neighbour
            match cell.get_unlinked_neighbours().choose(&mut rng) {
                // None only occurs when there are no unlinked neighbours. Move back to previous cell on stack.
                None => {
                    // Drop mutable reference to cell to allow item to be popped from stack.
                    drop(cell);
                    stack.pop();
                }

                // Link to chosen neighbour and add it to the top of the stack.
                Some(&dir) => {
                    let nb_rc = cell.get_neighbour(dir).unwrap();
                    cell.link(dir);

                    // Drop mutable reference to cell to allow nb_rc to be moved into stack.
                    drop(cell);
                    stack.push(nb_rc);
                }
            }
        }
    }
}
