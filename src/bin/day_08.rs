use std::time::Instant;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
enum Instruction {
    ADD,
    JMP,
    NOP,
}

fn read_lines() -> String {
    let mut content = String::new();
    match File::open("inputs/day_08.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }
    content
}

fn parse_instructions(lines: String) -> Vec<(Instruction, i16)> {
    lines
        .split("\n")
        .map(|line| {
            let mut line = line.splitn(2, " ");

            let instruction = match line.next().unwrap() {
                "acc" => Instruction::ADD,
                "jmp" => Instruction::JMP,
                "nop" => Instruction::NOP,
                _ => panic!("Unsupported instruction"),
            };

            let argument = line.next().unwrap().parse::<i16>().unwrap();

            (instruction, argument)
        })
        .collect()
}

pub fn part_one() -> i16 {
    let lines = read_lines();
    let instructions = parse_instructions(lines);

    let mut acc: i16 = 0;
    let mut pointer = 0;
    let mut executed_instructions = HashSet::new();

    while pointer < instructions.len() {
        let (instruction, argument) = instructions[pointer];
        match instruction {
            Instruction::ADD => {
                acc += argument;
                pointer += 1;
            }
            Instruction::JMP => {
                pointer += argument as usize;
            }
            Instruction::NOP => {
                pointer += 1;
            }
        };
        if executed_instructions.contains(&pointer) {
            break;
        }
        executed_instructions.insert(pointer);
    }

    acc
}

pub fn part_two() -> i16 {
    let lines = read_lines();
    let instructions = parse_instructions(lines);

    'patch_loop: for patch in 0..instructions.len() {
        let mut acc: i16 = 0;
        let mut pointer = 0;
        let mut executed_instructions = HashMap::new();

        while pointer < instructions.len() {
            let (mut instruction, argument) = instructions[pointer];
            if pointer == patch {
                instruction = match instruction {
                    Instruction::JMP => Instruction::NOP,
                    Instruction::NOP => Instruction::JMP,
                    _ => continue 'patch_loop,
                }
            }

            let previous = executed_instructions.entry(pointer).or_insert(0);
            if *previous >= 2 {
                continue 'patch_loop;
            }

            *previous += 1;

            match instruction {
                Instruction::ADD => {
                    acc += argument;
                    pointer += 1;
                }
                Instruction::JMP => {
                    pointer += argument as usize;
                }
                Instruction::NOP => {
                    pointer += 1;
                }
            };
        }

        return acc;
    }

    panic!("No solution found")
}

pub fn main() {
    let now = Instant::now();
    println!("Day 8: Handheld Halting");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
