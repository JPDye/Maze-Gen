// External imports
use rand::prelude::*;

// Self imports
use crate::{Generator, RectGrid};

#[derive(Debug)]
pub struct HuntAndKill {}

impl Generator for HuntAndKill {
    fn gen(&self, maze: &mut RectGrid) {
        let mut rng = thread_rng();

        let mut cell_rc = maze.get_random_cell();

        while cell_rc.is_some() {
            let mut cell = cell_rc.as_ref().unwrap().borrow_mut();

            // Get a list of the current cells unvisited neighbours.
            let nb_dirs = cell.get_unlinked_neighbours();

            // Choose a random neighbour.
            match nb_dirs.choose(&mut rng) {
                // If a neighbour was chosen (i.e. an unvisited neighbour existed), link to it.
                Some(&nb_dir) => {
                    let nb_rc = cell.get_neighbour(nb_dir);

                    /*
                    println!(
                        "Neighbours: {:?} --> Linked: {:?} --> Unlinked: {:?} --> Choice: {:?}",
                        cell.get_neighbours(),
                        cell.get_linked(),
                        cell.get_unlinked(),
                        nb_dir
                    );
                     */

                    cell.link(nb_dir);

                    // Drop the mutable reference to the cell to allow 'cell_rc' to be reassigned.
                    drop(cell);
                    cell_rc = nb_rc;

                    // println!("{}", maze);
                }

                None => {
                    // Drop mutable reference to the cell to allow 'cell_rc' to be reassigned.
                    drop(cell);
                    cell_rc = None;

                    for c_rc in maze.iter_cell() {
                        let mut c = c_rc.borrow_mut();

                        // If the current cell is unvisited check if it has any visited neighborus.
                        if c.get_linked().is_empty() {
                            let visited_neighbours = c.get_linked_neighbours();

                            // If there are visited neighbours, link to one of them and set current cell as next cell.
                            if !visited_neighbours.is_empty() {
                                let nb_dir = visited_neighbours.choose(&mut rng).unwrap();
                                c.link(*nb_dir);

                                // Drop mutable reference to the cell to allow 'c_rc' to be reassigned.
                                drop(c);
                                cell_rc = Some(c_rc);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
}
