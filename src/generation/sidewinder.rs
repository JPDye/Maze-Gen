use rand::prelude::*;

use super::{Generator, SquareGrid};
use crate::grids::core::Direction;

/// For each cell randomly decide to link to the north or the east.
/// If east is chosen, add it to the run of cells.
/// If north is chosen, link north from a random cell in the run and clear the run.
/// If there are no cells to the north, link to the east.
/// If there are no cells to the east, link north.
pub struct Sidewinder;

impl<G: SquareGrid> Generator<G> for Sidewinder {
    fn gen(grid: &mut G) {
        let mut rng = rand::thread_rng();

        let mut run = Vec::new();
        for i in 0..grid.cells().len() {
            // Get the next cell and add it to the current run.
            let cell = &grid.cells()[i];
            run.push(i);

            // Check if the cell is at a north or east boundary.
            let at_east_wall = !cell.neighbours.contains(&Direction::E);
            let at_north_wall = !cell.neighbours.contains(&Direction::N);

            // Run should end at the east boundary. Run should end if it is randomly chosen to link north (if north exists).
            let should_end_run = at_east_wall || (!at_north_wall && rand::random());

            if should_end_run {
                if !at_north_wall {
                    let choice = run.choose(&mut rng).unwrap();
                    grid.link(*choice, Direction::N);
                }
                run.clear();
            } else {
                grid.link(i, Direction::E);
            }
        }
    }
}
