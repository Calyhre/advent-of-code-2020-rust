use std::time::Instant;

use std::cmp;
use std::fs::File;
use std::io::{prelude::*, BufReader};

trait SeatNumber {
    fn as_seat_number(&self) -> usize;
}

impl SeatNumber for String {
    fn as_seat_number(&self) -> usize {
        let mut seat_number = 0usize;
        for &bit in self.as_bytes().iter() {
            seat_number <<= 1;
            if bit == 'B' as u8 || bit == 'R' as u8 {
                seat_number |= 1;
            }
        }
        seat_number
    }
}

pub fn part_one() -> usize {
    let file = File::open("inputs/day_05.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max_seat = 0;
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let seat_number = line.as_seat_number();
        max_seat = cmp::max(max_seat, seat_number);
    });

    max_seat
}

pub fn part_two() -> usize {
    let file = File::open("inputs/day_05.txt").unwrap();
    let reader = BufReader::new(file);

    let mut seat_map: [bool; 1024] = [false; 1024];
    reader.lines().for_each(|line| {
        let line = line.unwrap();
        let seat_number = line.as_seat_number();
        seat_map[seat_number] = true;
    });

    for row in 1..126usize {
        for col in 0..7usize {
            let seat_number = row * 8 + col;
            if seat_map[seat_number] == false
                && seat_map[seat_number - 1]
                && seat_map[seat_number + 1]
            {
                return seat_number;
            }
        }
    }

    panic!("No empty seat found 🤔")
}

pub fn main() {
    let now = Instant::now();
    println!("Day 5: Binary Boarding");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}µs", now.elapsed().as_micros());
}
