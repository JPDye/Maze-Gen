// External imports
use rand::seq::SliceRandom;

// Self imports
use crate::cells::cell::Direction::*;
use crate::gen::generator::Generator;
use crate::grids::rect_grid::RectGrid;

#[derive(Debug)]
pub struct BinaryTree {}

impl Generator for BinaryTree {
    fn gen(&self, maze: &mut RectGrid) {
        for cell_rc in maze.iter_cell() {
            // Get mutable access to the cell.
            let mut cell = cell_rc.borrow_mut();

            // Create vector to store possible neighbours to link to.
            let mut possible = Vec::new();

            // If a northern neighbour exists, add it to the list of possible links.
            if cell.neighbours.get(&N).unwrap().is_some() {
                possible.push(N);
            }

            // If a eastern neighbour exists, add it to the list of possible links.
            if cell.neighbours.get(&E).unwrap().is_some() {
                possible.push(E)
            }

            // Choose a neighbour to link to.
            let choice = possible.choose(&mut rand::thread_rng());

            // If a neighbour was picked (possible there was no viable neighbour), link to it.
            if let Some(&dir) = choice {
                cell.link(dir)
            }
        }
    }
}
