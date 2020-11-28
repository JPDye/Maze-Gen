use mazes::gen::binary_tree::BinaryTree;
use mazes::gen::generator::Generator;

use mazes::grids::rect_grid::RectGrid;

fn main() {
    let mut maze = RectGrid::new(15, 15);
    BinaryTree::gen(&mut maze);

    println!("{}", maze);
}
