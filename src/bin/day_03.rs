use std::time::Instant;

use std::fs::File;
use std::io::{prelude::*, BufReader};

const TREE: u8 = '#' as u8;

pub fn part_one() -> usize {
    let file = File::open("inputs/day_03.txt").unwrap();
    let reader = BufReader::new(file);

    let slope = (3, 1);
    let mut encountered_trees = 0;
    let mut position = (0, 0);
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let trees = line.as_bytes();

        if trees[position.0] == TREE {
            encountered_trees += 1;
        }
        position.0 = (position.0 + slope.0) % 31;
        position.1 = position.1 + slope.1;
    });

    encountered_trees
}

pub fn part_two() -> usize {
    let file = File::open("inputs/day_03.txt").unwrap();
    let reader = BufReader::new(file);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut encountered_trees = [0, 0, 0, 0, 0];
    let mut positions = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];
    reader.lines().enumerate().for_each(|(i, line)| {
        let line = line.unwrap();
        let trees = line.as_bytes();

        for (si, mut position) in positions.iter_mut().enumerate() {
            if position.1 == i {
                if trees[position.0] == TREE {
                    encountered_trees[si] += 1;
                }
                position.0 = (position.0 + slopes[si].0) % 31;
                position.1 = position.1 + slopes[si].1;
            }
        }
    });

    encountered_trees.iter().product::<usize>()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 3: Toboggan Trajectory");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}µs", now.elapsed().as_micros());
}
