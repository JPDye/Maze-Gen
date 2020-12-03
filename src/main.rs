// Generation Algorithm Trait
use mazes::gen::generator::Generator;

// Generation Algorithms
use mazes::gen::aldous_broder::AldousBroder;
// use mazes::gen::binary_tree::BinaryTree;
use mazes::gen::hunt_and_kill::HuntAndKill;
// use mazes::gen::sidewinder::Sidewinder;
// use mazes::gen::wilsons::Wilsons;

// Maze
use mazes::grids::rect_grid::RectGrid;

fn main() {
    let mut maze = RectGrid::new(25, 25);
    AldousBroder::gen(&mut maze);

    // println!("{}", maze);

    maze.create_image(2, 600, 600)
        .save("./imgs/aldous_broder.png")
        .expect("error saving");
}
