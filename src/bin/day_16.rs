use std::time::Instant;

use std::fs::File;
use std::io::prelude::*;
use std::ops::Range;

type Field = (String, (Range<u16>, Range<u16>));
type Fields = Vec<Field>;
type Ticket = Vec<u16>;

struct WeightedField {
    name: String,
    weight: u8,
    index: usize,
}

fn parse_range(range: &str) -> Range<u16> {
    let mut range = range.splitn(2, "-");
    let start = range.next().unwrap().parse::<u16>().unwrap();
    let end = range.next().unwrap().parse::<u16>().unwrap() + 1;
    Range { start, end }
}

fn parse_input() -> (Fields, Ticket, Vec<Ticket>) {
    let mut content = String::new();
    match File::open("inputs/day_16.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }

    let mut content = content.split("\n\n");

    let mut fields: Fields = vec![];
    content.next().unwrap().split("\n").for_each(|line| {
        let mut line = line.splitn(2, ": ");
        let name = line.next().unwrap();
        let mut ranges = line.next().unwrap().splitn(2, " or ");
        let min_range = parse_range(ranges.next().unwrap());
        let max_range = parse_range(ranges.next().unwrap());
        fields.push((name.to_string(), (min_range, max_range)));
    });

    let ticket: Ticket = content
        .next()
        .unwrap()
        .split_at(13)
        .1
        .split(',')
        .map(|i| i.parse::<u16>().unwrap())
        .collect();

    let tickets: Vec<Ticket> = content
        .next()
        .unwrap()
        .split_at(16)
        .1
        .split('\n')
        .map(|line| -> Ticket {
            return line.split(',').map(|i| i.parse::<u16>().unwrap()).collect();
        })
        .collect();

    (fields, ticket, tickets)
}

pub fn part_one() -> u16 {
    let (fields, _ticket, tickets) = parse_input();

    tickets
        .iter()
        .map(|ticket| -> u16 {
            ticket
                .iter()
                .filter(|&v| {
                    fields
                        .iter()
                        .find(|(_name, (min, max))| min.contains(v) || max.contains(v))
                        .is_none()
                })
                .sum()
        })
        .sum()
}

pub fn part_two() -> u64 {
    let (fields, ticket, tickets) = parse_input();

    let valid_tickets = tickets.iter().filter(|&ticket| {
        ticket
            .iter()
            .find(|&v| {
                fields
                    .iter()
                    .find(|(_name, (min, max))| min.contains(v) || max.contains(v))
                    .is_none()
            })
            .is_none()
    });

    let mut counters = vec![vec![0u16; fields.len()]; fields.len()];

    for ticket in valid_tickets {
        for (ticket_field_index, ticket_field) in ticket.iter().enumerate() {
            for (field_index, (_field_name, field_ranges)) in fields.iter().enumerate() {
                if !field_ranges.0.contains(ticket_field) && !field_ranges.1.contains(ticket_field)
                {
                    counters[field_index][ticket_field_index] += 1;
                }
            }
        }
    }

    let mut weighted_fields: Vec<WeightedField> = fields
        .iter()
        .enumerate()
        .map(|(i, field)| WeightedField {
            name: field.0.clone(),
            weight: counters[i].iter().filter(|&&v| v == 0).count() as u8,
            index: i,
        })
        .collect();

    weighted_fields.sort_by(|a, b| a.weight.cmp(&b.weight));

    weighted_fields
        .iter()
        .fold(
            vec![None; weighted_fields.len()],
            |mut acc, weighted_field| {
                let index = counters[weighted_field.index]
                    .iter()
                    .enumerate()
                    .position(|(i, &a)| a == 0 && acc[i].is_none())
                    .expect("There is someone on my spot");
                acc[index] = Some(&weighted_field.name);
                acc
            },
        )
        .iter()
        .enumerate()
        .map(|(i, field)| {
            if field.unwrap().starts_with("departure ") {
                ticket[i]
            } else {
                1
            }
        })
        .fold(1, |acc, field| acc * (field as u64))
}

pub fn main() {
    let now = Instant::now();
    println!("Day 16: Ticket Translation");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
