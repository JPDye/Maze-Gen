use super::core::{Cell, Direction, SquareGrid};

use image::{ImageBuffer, Rgba, RgbaImage};

#[derive(Clone, Debug)]
pub struct Grid {
    pub rows: usize,
    pub cols: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut grid = Self {
            rows,
            cols,
            cells: vec![Cell::default(); rows * cols],
        };

        for i in 0..rows * cols {
            for &dir in &[Direction::N, Direction::E, Direction::W, Direction::S] {
                if grid.neighbour_index(i, dir).is_some() {
                    let cell = grid.cells.get_mut(i).unwrap();
                    cell.neighbours.insert(dir);
                }
            }
        }

        grid
    }
}

impl SquareGrid for Grid {
    fn rows(&self) -> usize {
        self.rows
    }

    fn cols(&self) -> usize {
        self.cols
    }

    fn cells(&self) -> &Vec<Cell> {
        &self.cells
    }

    fn neighbour(&self, idx: usize, dir: Direction) -> Option<&Cell> {
        let index = self.neighbour_index(idx, dir)?;
        self.cells.get(index)
    }

    fn neighbour_index(&self, idx: usize, dir: Direction) -> Option<usize> {
        match dir {
            Direction::E => {
                if (idx + 1) % self.cols == 0 {
                    return None;
                }
                Some(idx + 1)
            }

            Direction::W => {
                if idx % self.cols == 0 {
                    return None;
                }
                Some(idx - 1)
            }

            Direction::N => {
                if idx / self.rows == 0 {
                    return None;
                }
                Some(idx - self.cols)
            }

            Direction::S => {
                if idx / self.rows == self.rows - 1 {
                    return None;
                }
                Some(idx + self.cols)
            }
        }
    }

    fn linked_neighbours(&self, idx: usize) -> Vec<Direction> {
        let mut dirs = Vec::new();

        if self.cells.get(idx).is_none() {
            return dirs;
        }

        for dir in &self.cells[idx].neighbours {
            let nb = self.neighbour(idx, *dir).unwrap();
            if !nb.links.is_empty() {
                dirs.push(*dir)
            }
        }
        dirs
    }

    fn unlinked_neighbours(&self, idx: usize) -> Vec<Direction> {
        let mut dirs = Vec::new();

        if self.cells.get(idx).is_none() {
            return dirs;
        }

        for dir in &self.cells[idx].neighbours {
            let nb = self.neighbour(idx, *dir).unwrap();
            if nb.links.is_empty() {
                dirs.push(*dir);
            }
        }
        dirs
    }

    fn link(&mut self, idx: usize, dir: Direction) {
        &mut self.cells[idx].links.insert(dir);

        let other_idx = self.neighbour_index(idx, dir).unwrap();
        let other_dir = match dir {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::W => Direction::E,
            Direction::S => Direction::N,
        };

        &mut self.cells[other_idx].links.insert(other_dir);
    }

    fn save(&self) -> RgbaImage {
        let cell_size = 2;

        let height = (cell_size * self.rows + 1) as u32;
        let width = (cell_size * self.cols + 1) as u32;

        // Colours.
        let white = Rgba([255, 255, 255, 255]);
        let black = Rgba([0, 0, 0, 255]);

        // Create image buffer.
        let mut imgbuf = ImageBuffer::from_pixel(width, height, white);

        // Draw North and West outer boundaries.
        let top_right = (self.cols * cell_size) as u32;
        draw_line(&mut imgbuf, 0, top_right, 0, black, false); // North

        let bottom_left = (self.rows * cell_size) as u32;
        draw_line(&mut imgbuf, 0, bottom_left, 0, black, true); // West

        // Draw walls of each Cell.
        for row in 0..self.rows {
            for col in 0..self.cols {
                let cell = &self.cells[row * self.cols + col];

                let x1 = (cell_size * col) as u32;
                let x2 = (cell_size * (col + 1)) as u32;
                let y1 = (cell_size * row) as u32;
                let y2 = (cell_size * (row + 1)) as u32;

                // Make cell black if it has no links.
                if cell.links.is_empty() {
                    for x in x1 + 1..x2 {
                        for y in y1 + 1..y2 {
                            imgbuf.put_pixel(x, y, black);
                        }
                    }
                }

                // Draw south boundary of cell.
                if cell.links.get(&Direction::S).is_none() {
                    draw_line(&mut imgbuf, x1, x2, y2, black, false);
                }

                // Draw east boundary of Cell.
                if cell.links.get(&Direction::E).is_none() {
                    draw_line(&mut imgbuf, y1, y2, x2, black, true);
                }
            }
        }

        imgbuf
    }
}

fn draw_line(imgbuf: &mut RgbaImage, p1: u32, p2: u32, q: u32, c: image::Rgba<u8>, vertical: bool) {
    if vertical {
        for p in p1..p2 + 1 {
            imgbuf.put_pixel(q, p, c);
        }
    } else {
        for p in p1..p2 + 1 {
            imgbuf.put_pixel(p, q, c);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Direction::*;

    use std::collections::HashSet;
    use std::iter::FromIterator;

    #[test]
    fn nb_idx() {
        let grid = Grid::new(3, 3);

        assert_eq!(grid.neighbour_index(0, N), None);
        assert_eq!(grid.neighbour_index(0, W), None);
        assert_eq!(grid.neighbour_index(0, E), Some(1));
        assert_eq!(grid.neighbour_index(0, S), Some(3));

        assert_eq!(grid.neighbour_index(4, N), Some(1));
        assert_eq!(grid.neighbour_index(4, W), Some(3));
        assert_eq!(grid.neighbour_index(4, E), Some(5));
        assert_eq!(grid.neighbour_index(4, S), Some(7));

        assert_eq!(grid.neighbour_index(8, N), Some(5));
        assert_eq!(grid.neighbour_index(8, W), Some(7));
        assert_eq!(grid.neighbour_index(8, E), None);
        assert_eq!(grid.neighbour_index(8, S), None);
    }

    #[test]
    fn grid_creation() {
        let grid = Grid::new(3, 3);

        let received = &grid.cells[0].neighbours;
        let expected: HashSet<Direction> = HashSet::from_iter(vec![E, S]);
        assert_eq!(received, &expected);

        let received = &grid.cells[2].neighbours;
        let expected: HashSet<Direction> = HashSet::from_iter(vec![W, S]);
        assert_eq!(received, &expected);

        let received = &grid.cells[4].neighbours;
        let expected: HashSet<Direction> = HashSet::from_iter(vec![N, E, S, W]);
        assert_eq!(received, &expected);

        let received = &grid.cells[6].neighbours;
        let expected: HashSet<Direction> = HashSet::from_iter(vec![N, E]);
        assert_eq!(received, &expected);

        let received = &grid.cells[8].neighbours;
        let expected: HashSet<Direction> = HashSet::from_iter(vec![N, W]);
        assert_eq!(received, &expected);
    }
}
