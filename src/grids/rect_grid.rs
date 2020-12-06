// External imports
use colorous;
use image;
use rand::Rng;

// Std imports
use std::fmt;
use std::rc::Rc;

// Crate imports
use crate::{Cell, Direction, Direction::*, HardCellLink};

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
    /// Given the index of a cell in the maze, return the cell that exists at that index.
    pub fn get_cell(&self, index: usize) -> Option<HardCellLink> {
        let cell_ref = self.grid.get(index)?;
        let cell_rc = Rc::clone(&cell_ref);
        Some(cell_rc)
    }

    /// Given the row number and column number, return the Cell that exists at that position in the slice (or return None).
    pub fn get_cell_row_col(&self, row: usize, col: usize) -> Option<HardCellLink> {
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

    /// Return an Iterator over Maze. Provides each cell, one by one.
    pub fn iter_cell(&self) -> IterCell {
        IterCell::new(self)
    }

    /// Return an Iterator over Maze. Provides each row, one by one.
    pub fn iter_row(&self) -> IterRow {
        IterRow::new(self)
    }

    /// Breadth first search to generate array of distances for colouring.
    pub fn get_distances(&self) -> Vec<Option<usize>> {
        let mut distances = vec![None; self.cols * self.rows];

        // Pick start cell and place it in "current" vec
        let cell_rc = self.get_cell(0).expect("no cell at index 0");
        let mut current = vec![(cell_rc, 0)];

        let mut distance = 0;
        while !current.is_empty() {
            let mut next = Vec::new();

            for (cell_rc, index) in &current {
                distances[*index] = Some(distance);

                let cell = cell_rc.borrow();
                let nb_dirs = cell.get_linked();
                for dir in nb_dirs {
                    let nb_index = self.get_index_relative(*index, *dir).unwrap();
                    let nb_rc = cell.get_neighbour(*dir).unwrap();

                    if distances[nb_index].is_none() {
                        next.push((nb_rc, nb_index));
                    }
                }
            }
            distance += 1;
            current = next;
        }
        distances
    }

    /// Create an ImageBuffer from the maze.
    pub fn create_image(
        &self,
        cell_size: usize,
        final_x: u32,
        final_y: u32,
        colour: bool,
    ) -> image::RgbImage {
        // Set image dimensions.
        let img_x = cell_size * self.rows + 1;
        let img_y = cell_size * self.cols + 1;

        // Set colours.
        let bg = image::Rgb([255, 255, 255]);
        let wall = image::Rgb([0, 0, 0]);

        // Calculate distances.
        let cycles = 1;
        let mut max = 0;
        let mut distances = Vec::new();
        let gradient = colorous::MAGMA;

        if colour {
            distances = self.get_distances();
            max = 1 + distances.iter().max().unwrap().unwrap() / cycles;
        }

        // Create ImageBuffer.
        let mut imgbuf = image::ImageBuffer::from_pixel(img_x as u32, img_y as u32, bg);

        if colour {
            for row in 0..self.rows {
                for col in 0..self.cols {
                    let x1 = col * cell_size;
                    let y1 = row * cell_size;
                    let x2 = (col + 1) * cell_size;
                    let y2 = (row + 1) * cell_size;

                    // Draw cell background.
                    let distance = distances[row * self.cols + col].unwrap();
                    let colour = gradient.eval_rational(max - (distance % max), max);
                    let cell_bg = image::Rgb([colour.r, colour.g, colour.b]);

                    for x in x1..x2 {
                        for y in y1..y2 {
                            imgbuf.put_pixel(x as u32, y as u32, cell_bg);
                        }
                    }
                }
            }
        }

        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell_rc = self.get_cell_row_col(row, col).unwrap();
                let cell = cell_rc.borrow_mut();

                let x1 = col * cell_size;
                let y1 = row * cell_size;
                let x2 = (col + 1) * cell_size;
                let y2 = (row + 1) * cell_size;

                // Draw Boundary. West wall.
                for y in 0..self.cols * cell_size + 1 {
                    imgbuf.put_pixel(0, y as u32, wall);
                }

                // Draw Boundary. South wall.
                for y in 0..self.rows * cell_size + 1 {
                    imgbuf.put_pixel(y as u32, (self.rows * cell_size) as u32, wall);
                }

                // Draw line for North wall of cell. From (x1, y1) to (x2, y1).
                if !cell.is_linked(N) {
                    for x in x1..x2 + 1 {
                        imgbuf.put_pixel(x as u32, y1 as u32, wall);
                    }
                }

                // Draw line for East wall of cell. (from x2, y1) to (x2, y2).
                if !cell.is_linked(E) {
                    for y in y1..y2 + 1 {
                        imgbuf.put_pixel(x2 as u32, y as u32, wall);
                    }
                }
            }
        }
        image::imageops::resize(
            &imgbuf,
            final_x,
            final_y,
            image::imageops::FilterType::Nearest,
        )
    }
}

/// Cell Iterator for the Maze.
pub struct IterCell<'a> {
    maze: &'a RectGrid,
    index: Option<usize>,
}

impl<'a> IterCell<'a> {
    pub fn new(maze: &'a RectGrid) -> Self {
        IterCell { maze, index: None }
    }
}

impl<'a> Iterator for IterCell<'a> {
    type Item = HardCellLink;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index.is_none() {
            self.index = Some(0)
        }
        let cell_ref = self.maze.grid.get(self.index.unwrap())?;
        let cell_rc = Rc::clone(cell_ref);
        self.index = Some(self.index.unwrap().checked_add(1)?);
        Some(cell_rc)
    }
}

impl<'a> DoubleEndedIterator for IterCell<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index.is_none() {
            self.index = Some(self.maze.grid.len() - 1);
        }

        let cell_ref = self.maze.grid.get(self.index.unwrap())?;
        let cell_rc = Rc::clone(cell_ref);
        self.index = Some(self.index.unwrap().checked_sub(1)?);
        Some(cell_rc)
    }
}

/// Row Iterator for the Maze.
pub struct IterRow<'a> {
    maze: &'a RectGrid,
    row: Option<usize>,
}

impl<'a> IterRow<'a> {
    pub fn new(maze: &'a RectGrid) -> Self {
        IterRow { maze, row: None }
    }
}

impl<'a> Iterator for IterRow<'a> {
    type Item = &'a [HardCellLink];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row.is_none() {
            self.row = Some(0);
        }

        if self.row.unwrap() < self.maze.rows {
            let start = self.row.unwrap() * self.maze.cols;
            let end = start + self.maze.cols;
            self.row = Some(self.row.unwrap() + 1);
            Some(&self.maze.grid[start..end])
        } else {
            None
        }
    }
}

impl<'a> DoubleEndedIterator for IterRow<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.row.is_none() {
            self.row = Some(self.maze.rows);
        }

        if self.row.unwrap() > 0 {
            let start = (self.row.unwrap() - 1) * self.maze.cols;
            let end = start + self.maze.cols;
            self.row = Some(self.row.unwrap() - 1);
            Some(&self.maze.grid[start..end])
        } else {
            None
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
