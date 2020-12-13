use std::time::Instant;

const T_0: i64 = 1000299;
const NOTES: &str = "41,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,37,x,x,x,x,x,971,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,23,x,x,x,x,x,29,x,487,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,19";

// Chinese Remainder Theorem algorithm found here https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn chinese_remainder(residues: &Vec<i64>, modulii: &Vec<i64>) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();

    let mut sum = 0;

    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

pub fn part_one() -> i64 {
    let bus = NOTES
        .split(',')
        .filter(|&id| id != "x")
        .map(|id| {
            let id = id.parse::<i64>().unwrap();
            let wait = id - (T_0 % id);
            (id, wait)
        })
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();
    bus.0 * bus.1
}

pub fn part_two() -> i64 {
    let buses: Vec<(i64, i64)> = NOTES
        .split(',')
        .enumerate()
        .map(|(index, id)| {
            if id == "x" {
                None
            } else {
                let id = id.parse::<i64>().unwrap();
                Some((((id - index as i64) % id) as i64, id))
            }
        })
        .filter(|a| a.is_some())
        .map(|a| a.unwrap())
        .collect();

    let modulii: Vec<i64> = buses.iter().map(|&b| b.1).collect();
    let residues: Vec<i64> = buses.iter().map(|&b| b.0).collect();

    chinese_remainder(&residues, &modulii).unwrap()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 13: Shuttle Search");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}µs", now.elapsed().as_micros());
}
