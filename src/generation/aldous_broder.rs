use rand::prelude::*;

use super::{Generator, SquareGrid};
use crate::grids::core::Direction;

/// Randomly move to any neighbour. If cell has been visited before, don't link.
pub struct AldousBroder;

impl<G: SquareGrid> Generator<G> for AldousBroder {
    fn gen(grid: &mut G) {
        let mut rng = rand::thread_rng();

        let mut idx = rng.gen_range(0..grid.cells().len());
        let mut cell = &grid.cells()[idx];

        let mut remaining = grid.cells().len() - 1;
        while remaining > 0 {
            // Pick a random direction.
            let dirs: Vec<Direction> = cell.neighbours.iter().cloned().collect();
            let dir = dirs.choose(&mut rng).unwrap();

            // Get index of neighbour in that direction.
            let nb_idx = grid.neighbour_index(idx, *dir).unwrap();

            // If it is unvisited, link it to the current cell.
            if grid.cells()[nb_idx].links.len() == 0 {
                grid.link(idx, *dir);
                remaining -= 1;
            }

            // Repeat with the neighbour as the current cell.
            idx = nb_idx;
            cell = &grid.cells()[nb_idx];
        }
    }
}
