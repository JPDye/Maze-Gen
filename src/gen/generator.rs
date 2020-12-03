use crate::grids::rect_grid::RectGrid;

pub trait Generator: std::fmt::Debug {
    fn gen(&self, grid: &mut RectGrid);
}
