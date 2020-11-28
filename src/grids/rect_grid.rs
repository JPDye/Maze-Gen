// External imports
use rand::Rng;

// Std imports
use std::fmt;
use std::rc::Rc;

// Crate imports
use crate::cells::cell::{Cell, Direction, Direction::*, HardCellLink};

/// Represents a maze. Contains a Vector of Cells and provides methods for interacting with them.
pub struct RectGrid {
    pub rows: usize,
    pub cols: usize,
    pub grid: Vec<HardCellLink>,
}

impl RectGrid {
    /// Create a maze of given size. All Cells will be completely surroudned by walls.
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Vec::with_capacity(rows * cols);

        // Construct grid.
        for _ in 0..cols * rows {
            grid.push(Cell::new());
        }

        // Create maze.
        let rect_grid = RectGrid { rows, cols, grid };

        // Point Cells to neighbours.
        for (idx, cell) in rect_grid.grid.iter().enumerate() {
            let mut cell = cell.borrow_mut();

            // For each direction, find neighbour in that Direction (Some or None) and point Cell there.
            for d in vec![N, S, E, W] {
                match rect_grid.get_cell_relative(idx, d) {
                    Some(rc) => cell.neighbours.insert(d, Some(Rc::downgrade(&rc))),
                    None => cell.neighbours.insert(d, None),
                };
            }
        }
        rect_grid
    }

    /// Get a pointer to a random cell within the grid. Returns option since grid can be 0 by 0.
    pub fn get_random_cell(&self) -> Option<HardCellLink> {
        let mut rng = rand::thread_rng();
        let row = rng.gen_range(0, self.rows);
        let col = rng.gen_range(0, self.cols);
        let cell = self.grid.get(row * self.cols + col)?;
        Some(Rc::clone(&cell))
    }

    /// Given the row number and column number, return the Cell that exists at that position in the slice (or return None).
    pub fn get_cell(&self, row: usize, col: usize) -> Option<HardCellLink> {
        let cell_ref = self.grid.get(row * self.cols + col)?;
        let cell_rc = Rc::clone(&cell_ref);
        Some(cell_rc)
    }

    /// Given the index of the current cell and a direction, return a pointer to the Cell that exists in that direction. Return None if Cell that lies in that direction doesn't exist.
    pub fn get_cell_relative(&self, idx: usize, d: Direction) -> Option<HardCellLink> {
        let cell_ref = self.grid.get(self.get_index_relative(idx, d)?)?;
        let cell_rc = Rc::clone(&cell_ref);
        Some(cell_rc)
    }

    /// Given the index of the current cell and a direction, return the index of the Cell that exists in that direction. Return None if Cell in that direction doesn't exist.
    pub fn get_index_relative(&self, idx: usize, d: Direction) -> Option<usize> {
        match d {
            // Get index of cell to north. One row above (same as the number of columns back in a 1d array).
            N => idx.checked_sub(self.cols),

            // Get index of cell to south. One row below. Ensure index does not exceed bounds.
            S => match idx.checked_add(self.cols) {
                Some(idx) if idx < self.rows * self.cols => Some(idx),
                _ => None,
            },

            // Get index of cell to east. Check if it is in same row as current cell (cells aren't adjacent if not).
            E => match idx.checked_add(1) {
                Some(idx) if idx / self.cols == (idx - 1) / self.cols => Some(idx),
                _ => None,
            },

            // Get index of cell to east. Must be in same row as current cell. Ensure does not exceed bounds.
            W => match idx.checked_sub(1) {
                Some(idx)
                    if idx < self.rows * self.cols && idx / self.cols == (idx + 1) / self.cols =>
                {
                    Some(idx)
                }
                _ => None,
            },
        }
    }
}

/// Implement Display trait for RectGrid. Creates (and outputs) string representation of the maze.
impl fmt::Display for RectGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = "+".to_string() + &"---+".repeat(self.cols) + "\n";

        for i in (0..self.rows * self.cols).step_by(self.cols) {
            let mut top = "|".to_string();
            let mut bot = "+".to_string();
            let corner = "+";

            for cell in &self.grid[i..i + self.cols] {
                let body = "   ";

                let east_boundary = match cell.borrow_mut().is_linked(E) {
                    true => " ",
                    false => "|",
                };

                top.push_str(body);
                top.push_str(east_boundary);

                let south_boundary = match cell.borrow_mut().is_linked(S) {
                    true => "   ",
                    false => "---",
                };

                bot.push_str(south_boundary);
                bot.push_str(corner);
            }
            top.push_str("\n");
            bot.push_str("\n");

            output.push_str(&top);
            output.push_str(&bot);
        }
        write!(f, "{}", output)
    }
}
