use crate::RectGrid;

pub trait Generator: std::fmt::Debug {
    fn gen(&self, grid: &mut RectGrid);
}
