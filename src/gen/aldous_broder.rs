// External imports
use rand::prelude::*;

// Self imports
use crate::gen::generator::Generator;
use crate::grids::rect_grid::RectGrid;

#[derive(Debug)]
pub struct AldousBroder {}

impl Generator for AldousBroder {
    fn gen(&self, maze: &mut RectGrid) {
        let mut rng = thread_rng();

        let mut cell_rc = maze.get_random_cell().unwrap();

        let mut unvisited = maze.grid.len() - 1;
        while unvisited > 0 {
            // Pick a direction from list of directions neighbours lie in.
            let nb_dirs = cell_rc.borrow().get_neighbours();
            let nb_dir = nb_dirs.choose(&mut rng).unwrap();

            // Get the neighbour in the chosen direction.
            let nb_rc = cell_rc.borrow().get_neighbour(*nb_dir).unwrap();

            // If the neighbour is not linked to any cells, link it to the current cell.
            if nb_rc.borrow().get_linked().len() == 0 {
                cell_rc.borrow_mut().link(*nb_dir);
                unvisited -= 1;
            }

            // Repeat with the neighbour as the current cell.
            cell_rc = nb_rc;
        }
    }
}
