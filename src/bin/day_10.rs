use std::time::Instant;

use std::collections::HashMap;

const INPUT: [u8; 91] = [
    95, 43, 114, 118, 2, 124, 120, 127, 140, 21, 66, 103, 102, 132, 136, 93, 59, 131, 32, 9, 20,
    141, 94, 109, 143, 142, 65, 73, 27, 83, 133, 104, 60, 110, 89, 29, 78, 49, 76, 16, 34, 17, 105,
    98, 15, 106, 4, 57, 1, 67, 71, 14, 92, 39, 68, 125, 113, 115, 26, 33, 61, 45, 46, 11, 99, 7,
    25, 130, 42, 3, 10, 54, 44, 139, 50, 8, 58, 86, 64, 77, 35, 79, 72, 36, 80, 126, 28, 123, 119,
    51, 22,
];

pub fn part_one() -> usize {
    let mut input = INPUT.clone();
    input.sort();

    let mut singles = 1usize;
    let mut triples = 1usize;

    for i in 1..input.len() {
        match input[i] - input[i - 1] {
            3 => {
                triples += 1;
            }

            2 => {
                continue;
            }

            1 => {
                singles += 1;
            }
            _ => return singles * triples,
        }
    }

    singles * triples
}

fn count_solutions(mut cache: &mut HashMap<u8, u64>, input: &[u8], parent: u8) -> u64 {
    if input.is_empty() {
        return 1;
    }

    let mut count: u64 = 0;
    for i in 0..3.min(input.len()) {
        if input[i] - parent <= 3 {
            let child = input[i];
            if let Some(&solution) = cache.get(&child) {
                count += solution;
            } else {
                let solution = count_solutions(&mut cache, &input[i + 1..], child);
                cache.insert(child, solution);
                count += solution;
            }
        }
    }

    count
}

pub fn part_two() -> u64 {
    let mut input = INPUT.clone();
    input.sort();

    let mut cache: HashMap<u8, u64> = HashMap::new();
    count_solutions(&mut cache, &input, 0)
}

pub fn main() {
    let now = Instant::now();
    println!("Day 10: Adapter Array");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}µs", now.elapsed().as_micros());
}
