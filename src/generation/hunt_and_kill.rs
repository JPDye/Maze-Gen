use rand::prelude::*;

use super::{Generator, SquareGrid};

pub struct HuntAndKill;

impl<G: SquareGrid> Generator<G> for HuntAndKill {
    fn gen(grid: &mut G) {
        let mut rng = rand::thread_rng();

        // Start the algorithm from a random cell.
        let mut idx = rng.gen_range(0..grid.cells().len());

        'outer: loop {
            // Get list of unvisited neighbours.
            let unvisited = grid.unlinked_neighbours(idx);

            // Choose a random unvisited random neighbour.
            match unvisited.choose(&mut rng) {
                // Link current cell to the neighbour.
                Some(dir) => {
                    grid.link(idx, *dir);
                    idx = grid.neighbour_index(idx, *dir).unwrap();
                }

                // Hunt for the first unvisited cell with a visited neighbour.
                None => {
                    for (i, cell) in grid.cells().iter().enumerate() {
                        // If cell is unvisited.
                        if cell.links.is_empty() {
                            // Get list of visited neighbours.
                            let visited = grid.linked_neighbours(i);

                            // Randomly choose neighbour from list.
                            match visited.choose(&mut rng) {
                                // Link to that neighbour and start from beginning.
                                Some(dir) => {
                                    grid.link(i, *dir);
                                    idx = i;
                                    continue 'outer;
                                }

                                // No unvisited neighbours existed.
                                _ => (),
                            }
                        }
                    }

                    break 'outer;
                }
            }
        }
    }
}
