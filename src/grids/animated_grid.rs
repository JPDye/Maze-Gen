use super::core::{Animated, Cell, Direction, SquareGrid};
use super::grid::Grid;

use std::fs::File;

use pbr::ProgressBar;

use image::codecs::gif::GifEncoder;
use image::imageops::FilterType;
use image::{Frame, RgbaImage};

/// Wraps a Grid. Tracks all links made by algorithm for replaying during animation.
pub struct AnimatedGrid {
    grid: Grid,
    links: Vec<(usize, Direction)>,
}

impl AnimatedGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            grid: Grid::new(rows, cols),
            links: Vec::new(),
        }
    }
}

impl SquareGrid for AnimatedGrid {
    fn rows(&self) -> usize {
        self.grid.rows()
    }

    fn cols(&self) -> usize {
        self.grid.cols()
    }

    fn cells(&self) -> &Vec<Cell> {
        self.grid.cells()
    }

    fn neighbour(&self, idx: usize, dir: Direction) -> Option<&Cell> {
        self.grid.neighbour(idx, dir)
    }

    fn neighbour_index(&self, idx: usize, dir: Direction) -> Option<usize> {
        self.grid.neighbour_index(idx, dir)
    }

    fn linked_neighbours(&self, idx: usize) -> Vec<Direction> {
        self.grid.linked_neighbours(idx)
    }

    fn unlinked_neighbours(&self, idx: usize) -> Vec<Direction> {
        self.grid.unlinked_neighbours(idx)
    }

    fn link(&mut self, idx: usize, dir: Direction) {
        self.links.push((idx, dir));
        self.grid.link(idx, dir);
    }

    fn save(&self) -> RgbaImage {
        self.grid.save()
    }
}

impl Animated for AnimatedGrid {
    fn animate(&self, path: &str) {
        let duration = 10;
        let fps = 25;
        let total_frames = fps * duration;
        let total_links = self.links.len();
        let links_per_frame = total_links / total_frames;

        assert_ne!(links_per_frame, 0);

        let file = File::create(path).expect("could not create file");
        let mut encoder = GifEncoder::new(file);

        let mut new_grid = Grid::new(self.rows(), self.cols());

        let mut pb = ProgressBar::new(total_frames as u64);
        pb.format("╢▌▌░╟");

        for chunk in self.links.chunks(links_per_frame) {
            for (idx, dir) in chunk {
                new_grid.link(*idx, *dir);
            }

            let image =
                image::imageops::resize(&mut new_grid.save(), 600, 600, FilterType::Nearest);

            let frame = Frame::new(image);
            encoder.encode_frame(frame).expect("unable to encode frame");

            pb.inc();
        }
        pb.finish();
    }
}
