use std::time::Instant;

use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Seat {
    FLOOR,
    EMPTY,
    OCCUPIED,
}

fn read_map() -> [[Seat; 91]; 94] {
    let mut map = [[Seat::FLOOR; 91]; 94];

    let file = File::open("inputs/day_11.txt").unwrap();
    let reader = BufReader::new(file);
    reader.lines().enumerate().for_each(|(i, line)| {
        line.unwrap().chars().enumerate().for_each(|(j, seat)| {
            map[i][j] = match seat {
                'L' => Seat::EMPTY,
                '#' => Seat::OCCUPIED,
                '.' => Seat::FLOOR,
                _ => panic!("Wrong letter"),
            }
        });
    });

    map
}

type Map = [[Seat; 91]; 94];
type Rule = Box<dyn Fn(usize, usize) -> (usize, usize)>;
type Rules = Vec<Rule>;

fn find_occupied_seat(
    map: &Map,
    rules: &Rules,
    tolerances: (usize, usize),
    find_adjacents: fn(&Map, &Rule, (usize, usize), (usize, usize)) -> usize,
) -> usize {
    let mut map = map.clone();
    let mut changed = true;
    let limits = (map.len(), map[0].len());

    while changed {
        changed = false;
        let mut map_after = map.clone();

        for i in 0..map.len() {
            for j in 0..(map[0].len()) {
                let seat = map[i][j];

                if seat == Seat::FLOOR {
                    continue;
                }

                let mut adjacent_occupied_seats = 0;
                for rule in rules.iter() {
                    adjacent_occupied_seats += find_adjacents(&map, &rule, limits, (i, j));
                }

                if seat == Seat::EMPTY && adjacent_occupied_seats == tolerances.0 {
                    map_after[i][j] = Seat::OCCUPIED;
                    changed = true;
                } else if seat == Seat::OCCUPIED && adjacent_occupied_seats >= tolerances.1 {
                    map_after[i][j] = Seat::EMPTY;
                    changed = true;
                }
            }
        }
        // print_map(&map_after);
        map = map_after;
    }

    map.iter()
        .map(|&r| r.iter().filter(|&&s| s == Seat::OCCUPIED).count())
        .sum()
}

pub fn part_one(map: &Map, rules: &Rules) -> usize {
    find_occupied_seat(
        map,
        rules,
        (0, 4),
        |map: &Map, rule: &Rule, limits: (usize, usize), xy: (usize, usize)| {
            let xy = rule(xy.0, xy.1);
            if xy.0 < limits.0 && xy.1 < limits.1 && map[xy.0][xy.1] == Seat::OCCUPIED {
                1
            } else {
                0
            }
        },
    )
}

pub fn part_two(map: &Map, rules: &Rules) -> usize {
    find_occupied_seat(
        map,
        rules,
        (0, 5),
        |map: &Map, rule: &Rule, limits: (usize, usize), xy: (usize, usize)| {
            let mut xy = rule(xy.0, xy.1);
            let mut sum = 0;
            while xy.0 < limits.0 && xy.1 < limits.1 {
                if map[xy.0][xy.1] == Seat::OCCUPIED {
                    sum += 1;
                    return sum;
                } else if map[xy.0][xy.1] == Seat::EMPTY {
                    return sum;
                }
                xy = rule(xy.0, xy.1);
            }
            sum
        },
    )
}

pub fn main() {
    let now = Instant::now();

    let map = read_map();
    let rules: Rules = vec![
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x - 1, y) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x - 1, y - 1) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x, y - 1) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x + 1, y - 1) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x + 1, y) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x + 1, y + 1) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x, y + 1) }),
        Box::new(move |x: usize, y: usize| -> (usize, usize) { (x - 1, y + 1) }),
    ];

    println!("Day 11: Seating System");
    println!("  Part one: {}", part_one(&map, &rules));
    println!("  Part two: {}", part_two(&map, &rules));
    println!("  Time: {}ms", now.elapsed().as_millis());
}
