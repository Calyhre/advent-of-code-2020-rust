use std::time::Instant;

use std::fs::File;
use std::io::{prelude::*, BufReader};

lalrpop_mod!(pub day_18_lexer);

pub fn part_one() -> i64 {
    let file = File::open("inputs/day_18.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            day_18_lexer::PartOneParser::new()
                .parse(line.unwrap().as_str())
                .unwrap()
        })
        .sum::<i64>()
}

pub fn part_two() -> i64 {
    let file = File::open("inputs/day_18.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            day_18_lexer::PartTwoParser::new()
                .parse(line.unwrap().as_str())
                .unwrap()
        })
        .sum::<i64>()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 18: Operation Order");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
