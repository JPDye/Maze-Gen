use rand::prelude::*;

use super::{Generator, SquareGrid};
use crate::grids::core::Direction;

/// Similar to Aldous-Broder algorithm. Perform a random walk but erase the walk if a previously
/// visited cell is encountered.
pub struct Wilsons;

impl<G: SquareGrid> Generator<G> for Wilsons {
    fn gen(grid: &mut G) {
        let mut rng = thread_rng();

        let mut visited = 1;

        let mut unvisited = vec![true; grid.cells().len()];
        let idx = rng.gen_range(0..unvisited.len());
        unvisited[idx] = false;

        let mut path: Vec<(usize, Direction)> = Vec::new();

        while visited < unvisited.len() {
            // Get a random cell and initialise new path.
            let mut idx = rng.gen_range(0..unvisited.len());
            let mut cell = &grid.cells()[idx];

            while unvisited[idx] {
                // Get a list of possible directions and choose one for the random walk.
                let dirs: Vec<Direction> = cell.neighbours.iter().cloned().collect();
                let dir = dirs.choose(&mut rng).unwrap();

                // Get the neighbouring cell and it's index.
                let nb_idx = grid.neighbour_index(idx, *dir).unwrap();
                let nb_cell = &grid.cells()[nb_idx];

                // If a loop has been formed, erase it.
                if let Some(pos) = path.iter().position(|(i, _)| *i == nb_idx) {
                    path = (&path[0..pos]).to_vec();
                } else {
                    path.push((idx, *dir));
                }

                // Set the neighbour as current cell for next iteration.
                cell = nb_cell;
                idx = nb_idx;
            }

            for (idx, dir) in &path {
                grid.link(*idx, *dir);
                unvisited[*idx] = false;
                visited += 1;
            }

            path.clear();
        }
    }
}

/*
use indexmap::IndexSet;

pub struct Wilsons;

impl<G: SquareGrid> Generator<G> for Wilsons {
    fn gen(grid: &mut G) {
        let mut rng = thread_rng();

        // Create set of all unvisited cell indexes.
        let mut unvisited: IndexSet<usize> = (0..grid.cells().len()).collect();

        let mut path: Vec<(usize, Direction)> = Vec::new();

        // Set a random cell as visited. Makes it target for first pass of algorithm.
        let rand_idx = rng.gen_range(0..unvisited.len());
        unvisited.swap_remove_index(rand_idx);

        while !unvisited.is_empty() {
            // Get a random unvisited cell.
            let mut set_idx = rng.gen_range(0..unvisited.len());
            let mut idx = *unvisited.get_index(set_idx).unwrap();
            let mut cell = &grid.cells()[idx];

            while unvisited.contains(&idx) {
                // Get a list of possible directions and choose one for the random walk.
                let dirs: Vec<Direction> = cell.neighbours.iter().cloned().collect();
                let dir = dirs.choose(&mut rng).unwrap();

                // Get the neighbouring cell and its index.
                let nb_idx = grid.neighbour_index(idx, *dir).unwrap();
                let nb_cell = &grid.cells()[nb_idx];

                // If a loop has been formed, erase it.
                if let Some(pos) = path.iter().position(|(i, _)| *i == nb_idx) {
                    path = (&path[0..pos]).to_vec();
                } else {
                    path.push((idx, *dir));
                }

                // Set the neighbouring cell as the current cell for next pass.
                idx = nb_idx;
                cell = nb_cell
            }

            // Link all cells in the path. Set them as visited.
            for (idx, dir) in &path {
                grid.link(*idx, *dir);
                unvisited.remove(idx);
            }

            path.clear();
        }
    }
}
*/
