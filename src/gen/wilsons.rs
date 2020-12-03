// External imports
use rand::prelude::*;

// Standard imports

// Self imports
use crate::cells::cell::Direction;
use crate::gen::generator::Generator;
use crate::grids::rect_grid::RectGrid;

#[derive(Debug)]
pub struct Wilsons {}

impl Generator for Wilsons {
    fn gen(&self, maze: &mut RectGrid) {
        // Initialise random number generator
        let mut rng = thread_rng();

        // Build vector of unvisited cell indices and counter for number of visited cells.
        let mut visited = 0;
        let mut unvisited = vec![true; maze.grid.len()];

        // Choose a random cell and set it as visited.
        let index = rng.gen_range(0, unvisited.len());
        unvisited[index] = false;
        visited += 1;

        let mut path: Vec<(usize, Direction)> = Vec::new();

        // Loop until all cells are visited.
        while visited < unvisited.len() {
            // Get a random cell and initialise a new path.
            let mut index = rng.gen_range(0, unvisited.len());
            let mut cell_rc = maze.get_cell(index).unwrap();

            // Loop while the chosen cell is unvisited.
            while unvisited[index] {
                // Pick a direction from list of directions a neighbour lies in.
                let nb_dirs = cell_rc.borrow().get_neighbours();
                let nb_dir = nb_dirs.choose(&mut rng).unwrap();

                // Get the neighbour and it's index.
                let nb_index = maze.get_index_relative(index, *nb_dir).unwrap();
                let nb_rc = cell_rc.borrow().get_neighbour(*nb_dir).unwrap();

                // Check if the chosen neighbour has been visited before.
                // If yes, move path back to when it first occured.
                // If no, add new neighbour to the path.
                if let Some(p) = path.iter().position(|(i, _)| *i == nb_index) {
                    path = (&path[0..p]).to_vec();
                } else {
                    path.push((index, *nb_dir));
                }

                // Set neighbour as the current cell.
                cell_rc = nb_rc;
                index = nb_index;
            }

            for (index, dir) in &path {
                // Get the cell and link it in specified direction.
                let cell_rc = maze.get_cell(*index).unwrap();
                let mut cell = cell_rc.borrow_mut();
                cell.link(*dir);

                // Set cell as visited.
                unvisited[*index] = false;
                visited += 1;
            }

            path.clear();
        }
    }
}
