// External imports
use rand::prelude::*;

// Standard imports
use std::rc::Rc;

// Self imports
use crate::{Direction::*, Generator, HardCellLink, RectGrid};

#[derive(Debug)]
pub struct Sidewinder {}

impl Generator for Sidewinder {
    fn gen(&self, maze: &mut RectGrid) {
        let mut rng = rand::thread_rng();

        // Iterate over every row
        for row in maze.iter_row() {
            // Vec to store all cells that make up the current "run".
            let mut run: Vec<HardCellLink> = Vec::new();

            // Iterate over every cell in the row
            for cell_ref in row {
                // Clone 'cell_ref' to get owned RC to current cell.
                let cell_rc = Rc::clone(cell_ref);

                // Check if cell is at the north or east boundary.
                let at_east_boundary = !cell_rc.borrow_mut().neighbour_exists(E);
                let at_north_boundary = !cell_rc.borrow_mut().neighbour_exists(N);

                // The run should end at the east boundary. Should also have a 50% chance to end if not at the north boundary.
                let should_end_run =
                    at_east_boundary || (!at_north_boundary && rng.gen_range(0, 2) == 1);

                // Add Cell to the 'run vector to make linking north logic simpler.
                run.push(Rc::clone(&cell_rc));

                // If run should end, link north if not at the north boundary. Clear the 'run' vec. Link east otherwise.
                if should_end_run {
                    if !at_north_boundary {
                        let choice = run.choose(&mut rng).unwrap();
                        choice.borrow_mut().link(N);
                    }
                    run = Vec::new();
                } else {
                    cell_rc.borrow_mut().link(E);
                }
            }
        }
    }
}
