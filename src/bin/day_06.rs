use std::time::Instant;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_logs() -> String {
    let mut content = String::new();
    match File::open("inputs/day_06.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }
    // Watch out for input file trailing new line!
    content.pop();
    content
}

pub fn part_one() -> usize {
    read_logs()
        .split("\n\n")
        .map(|answers| {
            let mut questions = HashSet::new();
            answers.split("\n").for_each(|answer| {
                answer.chars().for_each(|char| {
                    questions.insert(char);
                });
            });
            questions.len()
        })
        .sum::<usize>()
}

pub fn part_two() -> usize {
    read_logs()
        .split("\n\n")
        .map(|answers| {
            let mut questions = HashMap::new();
            let answers = answers.split("\n");
            let answers_count = answers
                .map(|pers| {
                    pers.chars().for_each(|char| {
                        *questions.entry(char).or_insert(0) += 1;
                    })
                })
                .count();

            let answers_count = questions
                .iter()
                .filter(|(&_c, &v)| v == answers_count)
                .count();

            answers_count
        })
        .sum::<usize>()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 6: Custom Customs");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
