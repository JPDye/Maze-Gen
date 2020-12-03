// External imports
use rand::prelude::*;

// Self imports
use crate::gen::generator::Generator;
use crate::grids::rect_grid::RectGrid;

pub struct HuntAndKill {}

impl Generator for HuntAndKill {
    fn gen(maze: &mut RectGrid) {
        let mut rng = thread_rng();

        let mut cell_rc = maze.get_random_cell();

        while cell_rc.is_some() {
            let mut cell = cell_rc.as_ref().unwrap().borrow_mut();

            // Get list of directions unlinked neighbours lie in and pick one direction.
            // If cell has an unvisited neighbour, link to it and set the neighbour as current cell.
            // If the cell has no unvisited neighbours, find the first unvisited cell that borders one visited cell.

            // let nb_dirs = cell.get_linked_neighbours();
            let nb_dirs = cell.get_unlinked();

            match nb_dirs.choose(&mut rng) {
                Some(&nb_dir) => {
                    // Get pointer to neighbour and link to current cell.

                    println!(
                        "NBs: {:?} --> Linked: {:?} --> Unlinked: {:?} --> Choice: {:?}",
                        cell.get_neighbours(),
                        cell.get_linked(),
                        cell.get_unlinked(),
                        nb_dir
                    );

                    let nb_rc = cell.get_neighbour(nb_dir);
                    cell.link(nb_dir);

                    // Drop mutable reference to cell to allow cell_rc to be reassigned.
                    drop(cell);
                    cell_rc = nb_rc;
                }

                None => {
                    // Drop mutable reference to cell to allow cell_rc to be reassigned.
                    drop(cell);
                    cell_rc = None;

                    // Find the first unvisited cell with a visited neighbour. First, loop over all cells.
                    for c_rc in maze.iter_cell() {
                        let mut c = c_rc.borrow_mut();

                        let mut visited_neighbours = Vec::new();

                        // If cell is unvisited, loop over all neighbours.
                        if c.get_linked().is_empty() {
                            for dir in c.get_neighbours() {
                                let nb_rc = c.get_neighbour(dir).unwrap();
                                let nb = nb_rc.borrow();

                                // If neighbour is visited, add to list of visited neighbours.
                                if nb.get_linked().len() > 0 {
                                    visited_neighbours.push(dir);
                                }
                            }

                            // If there were visited neighbours, randomly pick one and link it to the current cell.
                            if !visited_neighbours.is_empty() {
                                let nb_dir = visited_neighbours.choose(&mut rng).unwrap();
                                c.link(*nb_dir);

                                // Drop mutable reference to cell to allow cell_rc to be reassigned.
                                drop(c);
                                cell_rc = Some(c_rc);
                            }
                        }
                    }
                }
            }
        }
    }
}
