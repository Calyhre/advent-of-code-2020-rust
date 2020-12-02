use std::time::Instant;

use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_line(line: &String) -> (usize, usize, char, &str) {
    let (policy, password) = match line.split(": ").collect::<Vec<&str>>() {
        s => (s[0], s[1]),
    };

    match policy.split_whitespace().collect::<Vec<&str>>() {
        a => match a[0].split('-').collect::<Vec<&str>>() {
            b => (
                b[0].parse::<usize>().unwrap(),
                b[1].parse::<usize>().unwrap(),
                a[1].chars().nth(0).unwrap(),
                password,
            ),
        },
    }
}

pub fn part_one() -> usize {
    let file = File::open("inputs/day_02.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter(|line| {
            let line = match line {
                Ok(b) => b.clone(),
                _ => "".to_string(),
            };
            let (min, max, letter, password) = parse_line(&line);
            let found = password.split(letter).count() - 1;
            return found >= min && found <= max;
        })
        .count()
}

pub fn part_two() -> usize {
    let file = File::open("inputs/day_02.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .filter(|line| {
            let line = match line {
                Ok(b) => b.clone(),
                _ => "".to_string(),
            };
            let (first, second, letter, password) = parse_line(&line);
            let found_first = password.chars().nth(first - 1).unwrap() == letter;
            let found_second = password.chars().nth(second - 1).unwrap() == letter;
            return (found_first && !found_second) || (!found_first && found_second);
        })
        .count()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 2: Password Philosophy");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
