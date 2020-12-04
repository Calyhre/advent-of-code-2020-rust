use std::time::Instant;

use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;

fn read_logs() -> String {
    let mut content = String::new();
    match File::open("inputs/day_04.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }
    content
}

pub fn part_one() -> usize {
    let mut valid_passports = 0usize;
    read_logs().split("\n\n").for_each(|log| {
        let entries: Vec<&str> = log.split_whitespace().collect();
        let mut passport = HashSet::new();

        entries.iter().for_each(|&entry| {
            passport.insert(entry.split(":").next().unwrap());
        });

        let fields = passport.len();
        if fields == 8 || fields == 7 && !passport.contains("cid") {
            valid_passports += 1;
        }
    });

    valid_passports
}

pub fn part_two() -> usize {
    let mut valid_passports = 0usize;

    read_logs().split("\n\n").for_each(|log| {
        let entries: Vec<&str> = log.split_whitespace().collect();
        let mut passport = HashMap::new();

        entries.iter().for_each(|&entry| {
            let mut kv = entry.split(":");
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();
            passport.insert(key, value);
        });

        if let None = is_valid_height(passport.get("hgt")) {
            return;
        } else if let None = is_valid_passport_id(passport.get("pid")) {
            return;
        } else if let None = is_valid_eye_color(passport.get("ecl")) {
            return;
        } else if let None = is_between(passport.get("byr"), 1920, 2002) {
            return;
        } else if let None = is_between(passport.get("iyr"), 2010, 2020) {
            return;
        } else if let None = is_valid_hair_color(passport.get("hcl")) {
            return;
        } else if let None = is_between(passport.get("eyr"), 2020, 2030) {
            return;
        }

        valid_passports += 1;
    });

    valid_passports
}

fn is_between(value: Option<&&str>, min: u32, max: u32) -> Option<bool> {
    match value {
        Some(&value) => {
            let value = value.parse::<u32>().unwrap();
            if value < min || value > max {
                None
            } else {
                Some(true)
            }
        }
        None => None,
    }
}

fn is_valid_height(value: Option<&&str>) -> Option<bool> {
    match value {
        Some(&value) => {
            if value.ends_with("in") {
                let value = &value[0..value.len() - 2];
                is_between(Some(&value), 59, 76)
            } else if value.ends_with("cm") {
                let value = &value[0..value.len() - 2];
                is_between(Some(&value), 150, 193)
            } else {
                None
            }
        }
        None => None,
    }
}

fn is_valid_hair_color(value: Option<&&str>) -> Option<bool> {
    match value {
        Some(&value) => {
            if value.len() != 7 {
                None
            } else if value.chars().nth(0).unwrap() != '#' {
                None
            } else {
                match usize::from_str_radix(&value[1..], 16) {
                    Ok(_) => Some(true),
                    Err(_) => None,
                }
            }
        }
        None => None,
    }
}

const VALID_EYE_COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
fn is_valid_eye_color(value: Option<&&str>) -> Option<bool> {
    match value {
        Some(&value) => {
            if VALID_EYE_COLORS.contains(&value) {
                Some(true)
            } else {
                None
            }
        }
        None => None,
    }
}

fn is_valid_passport_id(value: Option<&&str>) -> Option<bool> {
    match value {
        Some(&value) => {
            if value.len() != 9 {
                None
            } else {
                match value.parse::<u64>() {
                    Ok(_) => Some(true),
                    Err(_) => None,
                }
            }
        }
        None => None,
    }
}

pub fn main() {
    let now = Instant::now();
    println!("Day 4: Passport Processing");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
