use std::{collections::HashSet, time::Instant};

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct Tile {
    id: u16,
    pixels: [u8; 100],
    borders: [u16; 8],
    connections: [Option<u16>; 8],
}

impl Tile {
    fn to_string(&self) -> String {
        let mut result = format!("Title {}\n", self.id);
        for i in 0..10 {
            for j in 0..10 {
                if self.pixels[i * 10 + j] == 1 {
                    result.push_str("\u{2588}\u{2588}")
                } else {
                    result.push_str("  ")
                }
            }
            result.push('\n')
        }

        result
    }

    fn rotate(&mut self) {
        let mut pixels = [0u8; 100];
        for i in 0..100 {
            pixels[(i * 10 % 100) + (9 - i / 10)] = self.pixels[i];
        }

        let mut connections = [None; 8];
        for i in 0..connections.len() {
            let base = i / 4 * 4;
            let direction = base + ((i + 1) % 4);
            connections[direction] = self.connections[i];
        }

        self.pixels = pixels;
        self.connections = connections;
    }

    fn flip(&mut self) {
        let mut pixels = [0u8; 100];
        for i in 0..10 {
            for j in 0..10 {
                pixels[i * 10 + j] = self.pixels[i * 10 + (9 - j)]
            }
        }

        self.pixels = pixels;
        self.connections = [
            self.connections[4],
            self.connections[7],
            self.connections[6],
            self.connections[5],
            self.connections[0],
            self.connections[3],
            self.connections[2],
            self.connections[1],
        ];
    }
}

fn read_tiles() -> Vec<Tile> {
    let mut content = String::new();
    match File::open("inputs/day_20.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }

    content
        .split("\n\n")
        .map(|raw_tile| {
            let mut raw_tile = raw_tile.splitn(2, ":\n");

            let id = raw_tile
                .next()
                .unwrap()
                .split_at(5)
                .1
                .parse::<u16>()
                .unwrap();

            let mut pixels = [0u8; 100];

            let mut i = 0usize;
            for pixel in raw_tile.next().unwrap().bytes() {
                if pixel == b'#' {
                    pixels[i] = 1;
                    i += 1;
                } else if pixel == b'.' {
                    i += 1;
                }
            }

            let mut borders: [u16; 8] = [0; 8];

            for i in 0..10 {
                borders[0] = (borders[0] << 1) + pixels[i] as u16;
                borders[1] = (borders[1] << 1) + pixels[i * 10 + 9] as u16;
                borders[2] = (borders[2] << 1) + pixels[(9 - i) + 90] as u16;
                borders[3] = (borders[3] << 1) + pixels[90 - i * 10] as u16;

                borders[4] = (borders[4] << 1) + pixels[9 - i] as u16;
                borders[5] = (borders[5] << 1) + pixels[99 - (i * 10)] as u16;
                borders[6] = (borders[6] << 1) + pixels[i + 90] as u16;
                borders[7] = (borders[7] << 1) + pixels[i * 10] as u16;
            }

            Tile {
                id,
                pixels,
                connections: [None; 8],
                borders,
            }
        })
        .collect()
}

fn print_image(image: &Vec<Vec<Tile>>) {
    for tile_row in image {
        if tile_row.is_empty() {
            continue;
        }
        for i in 0..10usize {
            for tile in tile_row {
                for (j, &pixel) in tile.pixels[(i * 10)..(i * 10 + 10)].iter().enumerate() {
                    let prefix: &str;
                    let suffix: &str;

                    if i == 0 || i == 9 || j == 0 || j == 9 {
                        prefix = "\u{1b}[31m";
                        suffix = "\u{1b}[0m";
                    } else {
                        prefix = "";
                        suffix = "";
                    }

                    if pixel == 1 {
                        print!("{}\u{2588}\u{2588}{}", prefix, suffix)
                    } else {
                        print!("  ")
                    }
                }
            }
            println!();
        }
    }
    println!("\n\n\n");
}

fn build_image() -> [[u8; 96]; 96] {
    let mut tiles = read_tiles();

    for i in 0..tiles.len() {
        'child_loop: for j in i..tiles.len() {
            if i == j {
                continue;
            }
            for (position_i, border) in tiles[i].borders.iter().enumerate() {
                let position_j = tiles[j].borders.iter().position(|b| b == border);
                if position_j.is_none() {
                    continue;
                }
                let position_j = position_j.unwrap();

                tiles[i].connections[position_i] = Some(tiles[j].id);
                tiles[j].connections[position_j] = Some(tiles[i].id);

                continue 'child_loop;
            }
        }
    }

    let mut first_corner = tiles
        .iter()
        .find(|t| t.connections.iter().filter(|c| c.is_some()).count() == 2)
        .unwrap()
        .clone();

    while first_corner.connections[1].is_none() || first_corner.connections[2].is_none() {
        first_corner.rotate();
    }

    let mut image: Vec<Vec<Tile>> = vec![Vec::new(); 12];
    image[0].push(first_corner);

    for x in 0..12 {
        'y_loop: for y in 0..12 {
            if x == 0 && y == 0 {
                continue 'y_loop;
            }

            let tile: &Tile;
            let next_tile_id: u16;

            if y == 0 {
                tile = &image[x - 1][0];
                next_tile_id = tile.connections[2].or(tile.connections[6]).unwrap();
            } else {
                tile = &image[x][y - 1];
                next_tile_id = tile.connections[1].or(tile.connections[5]).unwrap();
            }

            let mut next_tile = tiles.iter().find(|t| t.id == next_tile_id).unwrap().clone();

            let tile_position = tile
                .connections
                .iter()
                .position(|&p| p == Some(next_tile.id))
                .unwrap();

            let next_tile_position = next_tile
                .connections
                .iter()
                .position(|&p| p == Some(tile.id))
                .unwrap();

            let need_flip = (next_tile_position < 4 && tile_position < 4)
                || (next_tile_position >= 4 && tile_position >= 4);
            if need_flip {
                next_tile.flip();
            }

            let tile_position = tile
                .connections
                .iter()
                .position(|&p| p == Some(next_tile.id))
                .unwrap()
                % 4;

            let next_tile_position = next_tile
                .connections
                .iter()
                .position(|&p| p == Some(tile.id))
                .unwrap()
                % 4;

            let turns = (tile_position - next_tile_position + 6) % 4;

            for _i in 0..turns {
                next_tile.rotate();
            }

            image[x].push(next_tile);
        }
    }

    let mut pixels = [[0u8; 96]; 96];
    for (x, tile_row) in image.iter().enumerate() {
        for (y, tile) in tile_row.iter().enumerate() {
            for i in 0..8 {
                pixels[x * 8 + i][(y * 8)..((y * 8) + 8)]
                    .copy_from_slice(&tile.pixels[(((i + 1) * 10) + 1)..(((i + 1) * 10) + 9)]);
            }
        }
    }

    pixels
}

pub fn part_one() -> u64 {
    let mut tiles = read_tiles();

    for i in 0..tiles.len() {
        'child_loop: for j in i..tiles.len() {
            if i == j {
                continue;
            }
            for (position_i, border) in tiles[i].borders.iter().enumerate() {
                let position = tiles[j].borders.iter().position(|b| b == border);
                if position.is_none() {
                    continue;
                }
                let position_j = position.unwrap();

                tiles[i].connections[position_i] = Some(tiles[j].id);
                tiles[j].connections[position_j] = Some(tiles[i].id);

                continue 'child_loop;
            }
        }
    }

    tiles
        .iter()
        .filter(|t| t.connections.iter().filter(|c| c.is_some()).count() == 2)
        .fold(1, |acc, t| acc * t.id as u64)
}

pub fn part_two() -> usize {
    let image = build_image();

    for line in &image {
        for &pixel in line {
            if pixel == 1 {
                print!("\u{2588}\u{2588}")
            } else {
                print!("  ")
            }
        }
        println!();
    }

    panic!("Not done yet")
}

pub fn main() {
    let now = Instant::now();
    println!("Day 20: Jurassic Jigsaw");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
