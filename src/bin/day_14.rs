use std::time::Instant;

use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

type Mask = (u64, u64, u64, u64);

fn parse_mask(line: &String) -> Mask {
    let mask_str = line.split_at(7).1;
    let mut mask: Mask = (u64::MAX, 0u64, 0u64, 1u64);
    for (i, char) in mask_str.chars().enumerate() {
        match char {
            '0' => {
                mask.0 ^= 1u64 << mask_str.len() - (i + 1);
            }
            '1' => {
                mask.1 |= 1u64 << mask_str.len() - (i + 1);
            }
            'X' => {
                mask.2 |= 1u64 << mask_str.len() - (i + 1);
            }
            c => panic!("Invalid mask bit: {}", c),
        }
    }

    let mut total = mask.2;
    while total > 0 {
        mask.3 <<= total & 1;
        total >>= 1;
    }

    mask
}

fn parse_memory_write(line: &String) -> (u64, u64) {
    let mut split = line.splitn(2, "] = ");
    let address = split.next().unwrap().split_at(4).1.parse::<u64>().unwrap();
    let value = split.next().unwrap().parse::<u64>().unwrap();
    (address, value)
}

pub fn part_one() -> u64 {
    let file = File::open("inputs/day_14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut mask: Mask = (0, 0, 0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();

    reader.lines().for_each(|line| {
        let line = line.unwrap();

        if line.starts_with("mask") {
            mask = parse_mask(&line);
        } else {
            let (address, value) = parse_memory_write(&line);
            memory.insert(address, value & mask.0 | mask.1);
        };
    });

    memory.values().sum::<u64>()
}

pub fn part_two() -> u64 {
    let file = File::open("inputs/day_14.txt").unwrap();
    let reader = BufReader::new(file);

    let mut mask: Mask = (0, 0, 0, 0);
    let mut memory: HashMap<u64, u64> = HashMap::new();

    reader.lines().for_each(|line| {
        let line = line.unwrap();

        if line.starts_with("mask") {
            mask = parse_mask(&line.to_string());
        } else {
            let (address, value) = parse_memory_write(&line);
            let address = address | mask.1;
            let mut address_mask = 0u64;
            memory.insert(address_mask ^ address, value);
            for _ in 1..mask.3 {
                address_mask = ((!mask.2 | address_mask) + 1) & mask.2;
                memory.insert(address_mask ^ address, value);
            }
        };
    });

    memory.values().sum::<u64>()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 14: Docking Data");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
