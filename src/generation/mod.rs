use crate::grids::core::SquareGrid;

mod aldous_broder;
pub use aldous_broder::AldousBroder;

mod binary_tree;
pub use binary_tree::BinaryTree;

mod sidewinder;
pub use sidewinder::Sidewinder;

mod wilsons;
pub use wilsons::Wilsons;

mod hunt_and_kill;
pub use hunt_and_kill::HuntAndKill;

mod backtracker;
pub use backtracker::Backtracker;

mod ellers;
pub use ellers::Ellers;

mod division;
pub use division::Division;

pub trait Generator<G: SquareGrid> {
    fn gen(grid: &mut G);
}
