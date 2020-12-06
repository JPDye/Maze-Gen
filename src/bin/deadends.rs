// External imports
use std::collections::HashMap;

// Self imports
use mazes::{AldousBroder, Backtracker, BinaryTree, HuntAndKill, Sidewinder, Wilsons};
use mazes::{Generator, RectGrid};

fn main() {
    let tries = 25;
    let size = 25;

    let algorithms: Vec<Box<dyn Generator>> = vec![
        Box::new(BinaryTree {}),
        Box::new(Sidewinder {}),
        Box::new(AldousBroder {}),
        Box::new(Wilsons {}),
        Box::new(HuntAndKill {}),
        Box::new(Backtracker {}),
    ];
    let mut averages: HashMap<String, f64> = HashMap::new();

    for alg in algorithms.iter() {
        println!("Running: {:?}", alg);

        let mut dead_end_counts: Vec<usize> = Vec::new();

        for _ in 0..tries {
            let mut maze = RectGrid::new(25, 25);
            alg.gen(&mut maze);
            dead_end_counts.push(dead_end_counter(&maze));
        }

        let total_dead_ends = dead_end_counts.iter().fold(0, |acc, x| acc + x);
        averages.insert(
            format!("{:?}", alg),
            total_dead_ends as f64 / dead_end_counts.len() as f64,
        );
    }

    println!("\n-------\n");

    let total_cells = size * size;
    println!(
        "Average dead-ends per {}x{} maze ({} cells): ",
        size, size, total_cells
    );

    for (alg, avg) in averages.iter() {
        let formatted = format!("{:.*}", 1, (*avg / total_cells as f64) * 100.0);
        println!(
            "{}: {} out of {} ({:02}%)",
            alg, avg, total_cells, formatted
        );
    }
}

fn dead_end_counter(maze: &RectGrid) -> usize {
    let mut counter = 0;
    for cell_rc in maze.iter_cell() {
        if cell_rc.borrow().get_linked().len() == 1 {
            counter += 1;
        }
    }
    counter
}
