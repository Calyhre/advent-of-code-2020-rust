use std::time::Instant;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Point {
    north: isize,
    east: isize,
}

impl Point {
    fn rotate(&mut self, angle: isize) {
        let angle = angle as f64;

        let cos_angle = angle.to_radians().cos().round() as isize;
        let sin_angle = angle.to_radians().sin().round() as isize;
        let east = self.east;
        let north = self.north;
        self.east = east * cos_angle - north * sin_angle;
        self.north = east * sin_angle + north * cos_angle;
    }

    fn apply_vector(&mut self, vector: &Point, n: isize) {
        self.east += n * vector.east;
        self.north += n * vector.north;
    }

    fn manhattan_distance(&self) -> usize {
        return (self.north.abs() + self.east.abs()) as usize;
    }
}

pub fn part_one() -> usize {
    let file = File::open("inputs/day_12.txt").unwrap();
    let reader = BufReader::new(file);

    let mut location = Point { north: 0, east: 0 };
    let mut heading = Point { north: 0, east: 1 };

    reader.lines().for_each(|line| {
        let line = line.unwrap();

        let (command, argument) = match line.split_at(1) {
            (o, a) => (o.chars().next().unwrap(), a.parse::<isize>().unwrap()),
        };

        match command {
            'N' => location.north += argument,
            'E' => location.east += argument,
            'S' => location.north -= argument,
            'W' => location.east -= argument,

            'R' => heading.rotate(-argument),
            'L' => heading.rotate(argument),

            'F' => location.apply_vector(&heading, argument),
            _ => panic!("Unsupported command"),
        };
    });

    location.manhattan_distance()
}

pub fn part_two() -> usize {
    let file = File::open("inputs/day_12.txt").unwrap();
    let reader = BufReader::new(file);

    let mut location = Point { north: 0, east: 0 };
    let mut waypoint = Point { north: 1, east: 10 };

    reader.lines().for_each(|line| {
        let line = line.unwrap();

        let (command, argument) = match line.split_at(1) {
            (o, a) => (o.chars().next().unwrap(), a.parse::<isize>().unwrap()),
        };

        match command {
            'N' => waypoint.north += argument,
            'E' => waypoint.east += argument,
            'S' => waypoint.north -= argument,
            'W' => waypoint.east -= argument,

            'R' => waypoint.rotate(-argument),
            'L' => waypoint.rotate(argument),

            'F' => location.apply_vector(&waypoint, argument),
            _ => panic!("Unsupported command"),
        };
    });

    location.manhattan_distance()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 12: Rain Risk");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}µs", now.elapsed().as_micros());
}
