use rand::prelude::*;

use super::{Generator, SquareGrid};
use crate::grids::core::Direction;

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

pub struct Ellers;

impl<G: SquareGrid> Generator<G> for Ellers {
    fn gen(grid: &mut G) {
        let mut rng = rand::thread_rng();

        let mut state = RowState::default();

        for row in 0..grid.rows() {
            for col in 1..grid.cols() {
                // Get the index of the current cell and the previous cell.
                let cell = row * grid.cols() + col;
                let prev_cell = grid.neighbour_index(cell, Direction::W).unwrap();

                // Get the index (key) of the sets the current and previous cells resides in.
                let set = state.set_for(cell);
                let prev_set = state.set_for(prev_cell);

                // Decide if the cells should be linked.
                // If they are of different sets and not on the bottom row, randomly decide to link.
                // If they are of different sets and on the bottom row, always link them.
                let should_link = set != prev_set
                    && (grid.neighbour_index(cell, Direction::S).is_none() || rng.gen_bool(0.5));

                // Link the cells and merge the sets they are in.
                if should_link {
                    grid.link(cell, Direction::W);
                    state.merge(prev_set, set);
                }
            }

            // Link downwards if not on the last row.
            if row != grid.rows() - 1 {
                let mut next_state = RowState::new(state.next_set);

                // For each set, randomly link downwards at least once.
                for (set_key, set) in state.cells_in_set.iter() {
                    let mut linked = false;

                    // Randomly decide to link downwards from each cell.
                    for cell in set {
                        if rng.gen_bool(0.5) {
                            let south_cell = grid.neighbour_index(*cell, Direction::S).unwrap();
                            grid.link(*cell, Direction::S);

                            next_state.add(*set_key, south_cell);
                            linked = true;
                        }
                    }

                    // If no links were made, link once.
                    if !linked {
                        let cells = Vec::from_iter(set);
                        if let Some(cell) = cells.choose(&mut rng) {
                            let south_cell = grid.neighbour_index(**cell, Direction::S).unwrap();
                            grid.link(**cell, Direction::S);

                            next_state.add(*set_key, south_cell);
                        }
                    }
                }

                state = next_state;
            }
        }
    }
}

#[derive(Default)]
struct RowState {
    next_set: usize,
    cells_in_set: HashMap<usize, HashSet<usize>>,
    set_for_cell: HashMap<usize, usize>,
}

impl RowState {
    pub fn new(next_set: usize) -> Self {
        Self {
            next_set,
            cells_in_set: HashMap::new(),
            set_for_cell: HashMap::new(),
        }
    }

    // Add the given cell to the given set.
    pub fn add(&mut self, set: usize, cell: usize) {
        self.set_for_cell.insert(cell, set);

        self.cells_in_set
            .entry(set)
            .or_insert(HashSet::new())
            .insert(cell);
    }

    // Return the key of the set a Cell resides in. Insert the cell into a set if it is not in one already.
    pub fn set_for(&mut self, cell: usize) -> usize {
        if !self.set_for_cell.contains_key(&cell) {
            self.add(self.next_set, cell);
            self.next_set += 1;
        }

        self.set_for_cell[&cell]
    }

    pub fn merge(&mut self, key1: usize, key2: usize) {
        // Get the two sets.
        let set1 = self.cells_in_set.get(&key1).unwrap();
        let set2 = self.cells_in_set.get(&key2).unwrap();

        // Update HashMap tracking which cells are in which sets.
        for cell in set2 {
            self.set_for_cell.insert(*cell, key1);
        }

        // Merge sets, insert new set in place of old set, delete other set.
        let new_set = HashSet::from_iter(set1.union(set2).cloned());
        self.cells_in_set.insert(key1, new_set);
        self.cells_in_set.remove(&key2);
    }
}
