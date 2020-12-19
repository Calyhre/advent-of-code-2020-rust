use std::time::Instant;

use onig::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

fn build_regex(
    rule_id: u8,
    rule: &Vec<Vec<String>>,
    rules: &HashMap<u8, Vec<Vec<String>>>,
) -> String {
    let str_rule = rule
        .iter()
        .map(|subrule| {
            subrule
                .iter()
                .map(|subrule_part| {
                    if subrule_part.starts_with("\"") {
                        subrule_part.split("\"").nth(1).unwrap().to_string()
                    } else {
                        let sub_rule_id = subrule_part.parse::<u8>().unwrap();
                        if sub_rule_id == rule_id {
                            format!("\\g<g{}>", rule_id)
                        } else {
                            build_regex(sub_rule_id, rules.get(&sub_rule_id).unwrap(), &rules)
                        }
                    }
                })
                .collect::<Vec<String>>()
                .join("")
        })
        .collect::<Vec<String>>();
    format!("(?<g{}>{})", rule_id, str_rule.join("|"))
}

fn parse_input() -> (HashMap<u8, Vec<Vec<String>>>, Vec<String>) {
    let mut content = String::new();
    match File::open("inputs/day_19.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }

    let mut content = content.split("\n\n");

    let (rules, messages): (Vec<&str>, Vec<String>) = (
        content.next().unwrap().split("\n").collect(),
        content
            .next()
            .unwrap()
            .split("\n")
            .map(|s| s.to_string())
            .collect(),
    );

    let rules = rules.iter().fold(HashMap::new(), |mut acc, rule| {
        let mut rule = rule.splitn(2, ": ");
        let rule_id = rule.next().unwrap().parse::<u8>().unwrap();

        acc.insert(
            rule_id,
            rule.next()
                .unwrap()
                .split(" | ")
                .map(|l| l.split(" ").map(|s| s.to_string()).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>(),
        );
        return acc;
    });

    (rules, messages)
}

pub fn part_one() -> usize {
    let (rules, messages) = parse_input();
    let root = rules.get(&0u8).unwrap();
    let regex = Regex::new(format!("^{}$", build_regex(0, root, &rules)).as_str()).unwrap();

    messages
        .iter()
        .filter(|&message| regex.is_match(message.as_str()))
        .count()
}

pub fn part_two() -> usize {
    let (mut rules, messages) = parse_input();

    rules.insert(
        8,
        vec![
            vec![String::from("42")],
            vec![String::from("42"), String::from("8")],
        ],
    );

    rules.insert(
        11,
        vec![
            vec![String::from("42"), String::from("31")],
            vec![String::from("42"), String::from("11"), String::from("31")],
        ],
    );

    let root = rules.get(&0u8).unwrap();
    let regex = Regex::new(format!("^{}$", build_regex(0, root, &rules)).as_str()).unwrap();

    messages
        .iter()
        .filter(|&message| regex.is_match(message.as_str()))
        .count()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 19: Monster Messages");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
