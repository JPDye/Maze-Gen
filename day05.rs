// Standard imports
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

// Self imports
use crate::parts::{Part, Part::*};

pub fn run(part: Part) {
    let start = Instant::now();

    let file = File::open("./data/data05.txt").expect("file not found!");
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut min = std::usize::MAX;
    let mut seats = HashSet::new();

    for line in reader.lines() {
        // Convert input into a binary number.
        let mut id = 0;
        let mut exp = 9;
        for c in line.unwrap().chars() {
            if c.to_string() == "B" || c.to_string() == "R" {
                id += 2usize.pow(exp);
            }
            exp -= 1;
        }

        // Build hashset
        seats.insert(id);

        // Find minimum and maximum values
        if id < min {
            min = id;
        } else if id > max {
            max = id;
        }
    }

    let answer = match part {
        One => max,
        Two => *seats.difference(&(min..max).collect()).next().unwrap(),
    };

    println!("Answer: {} in {:?}", answer, start.elapsed());
}
