use rand::prelude::*;

use super::{Generator, SquareGrid};
use crate::grids::core::Direction;

pub struct Division;

impl<G: SquareGrid> Generator<G> for Division {
    fn gen(grid: &mut G) {
        todo!();
    }
}
