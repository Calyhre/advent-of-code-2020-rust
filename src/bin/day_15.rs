use std::time::Instant;

const INPUT: [u8; 7] = [16, 12, 1, 0, 15, 7, 11];

pub fn part_one(input: &[u8], limit: usize) -> u32 {
    let mut memory: Vec<u32> = vec![0; limit];

    for turn in 0..input.len() {
        memory[input[turn] as usize] = (turn + 1) as u32;
    }

    let mut last_number = input.last().unwrap().to_owned() as u32;
    for turn in input.len()..limit {
        let turn = turn + 1;
        let mut number = memory[last_number as usize];
        if number > 0 {
            number = (turn - 1) as u32 - number;
        }
        memory[last_number as usize] = (turn - 1) as u32;
        last_number = number;
    }

    last_number
}

pub fn part_two(input: &[u8]) -> u32 {
    part_one(input, 30000000)
}

pub fn main() {
    let now = Instant::now();
    println!("Day 15: Rambunctious Recitation");
    println!("  Part one: {}", part_one(&INPUT, 2020));
    println!("  Part two: {}", part_two(&INPUT));
    println!("  Time: {}ms", now.elapsed().as_millis());
}
