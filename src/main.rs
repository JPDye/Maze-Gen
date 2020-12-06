// Standard imports
use std::time::Instant;

// Self imports
use mazes::Generator;
use mazes::RectGrid;

fn main() {
    let start = Instant::now();

    let mut maze = RectGrid::new(200, 200);
    mazes::HuntAndKill {}.gen(&mut maze);
    maze.create_image(2, 1200, 1200, true)
        .save("./imgs/colourful.png")
        .expect("error saving");

    println!("Maze created in {:?}", start.elapsed());
}
