use std::time::Instant;

use std::convert::TryInto;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_numbers() -> [u64; 1000] {
    let file = File::open("inputs/day_09.txt").unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .as_slice()
        .try_into()
        .unwrap()
}

fn is_valid_sequence(sequence: u64, options: [u64; 25]) -> bool {
    for i in 0..25 {
        for j in i..25 {
            if options[i] + options[j] == sequence {
                return true;
            }
        }
    }
    return false;
}

fn find_first_invalid(numbers: &[u64; 1000]) -> u64 {
    for i in 25..numbers.len() {
        if !is_valid_sequence(numbers[i], numbers[(i - 25)..i].try_into().unwrap()) {
            return numbers[i];
        }
    }

    panic!("Can't find an invalid number")
}

pub fn part_one() -> u64 {
    let numbers = read_numbers();
    find_first_invalid(&numbers)
}

pub fn part_two() -> u64 {
    let numbers = read_numbers();
    let invalid = find_first_invalid(&numbers);

    let mut min_index = 0usize;
    for max_index in 0..numbers.len() {
        let mut value: u64 = numbers[min_index..max_index].iter().sum();

        while value > invalid {
            min_index += 1;
            value = numbers[min_index..max_index].iter().sum();
        }

        if value == invalid {
            let min = numbers[min_index..max_index].iter().min().unwrap();
            let max = numbers[min_index..max_index].iter().max().unwrap();
            return min + max;
        }
    }

    panic!("Can't find a solution")
}

pub fn main() {
    let now = Instant::now();
    println!("Day 9: Encoding Error");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}µs", now.elapsed().as_micros());
}
