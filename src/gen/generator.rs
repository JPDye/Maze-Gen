use crate::grids::rect_grid::RectGrid;

pub trait Generator {
    fn gen(grid: &mut RectGrid);
}
