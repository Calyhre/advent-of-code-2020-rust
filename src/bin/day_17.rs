use std::time::Instant;

const INPUT: &str = "####.#..
.......#
#..#####
.....##.
##...###
#..#.#.#
.##...#.
#...##..";

type StateLayerX = Vec<u8>;
type StateLayerY = Vec<StateLayerX>;
type StateLayerZ = Vec<StateLayerY>;
type State3D = StateLayerZ;
type State4D = Vec<StateLayerZ>;

fn get_boundaries(new_index: usize, layer_size: usize) -> (usize, usize) {
    let min;
    let max;

    if new_index <= 1 {
        min = 0;
    } else {
        min = new_index - 2;
    }

    if new_index >= layer_size + 2 {
        max = layer_size;
    } else {
        max = (new_index + 1).min(layer_size);
    }

    (min, max)
}

fn state_3d_from_str(input: &str) -> State3D {
    let mut state: State3D = vec![];

    state.push(Vec::new());
    input.split("\n").enumerate().for_each(|(y, line)| {
        state[0].push(Vec::new());
        line.bytes().for_each(|p| {
            if p == b'#' {
                state[0][y].push(1)
            } else {
                state[0][y].push(0)
            }
        });
    });

    state
}

fn run_cycle_part_one(state: &State3D) -> State3D {
    let z_size = state.len();
    let y_size = state[0].len();
    let x_size = state[0][0].len();

    let mut new_state: State3D = vec![vec![vec![0; x_size + 2]; y_size + 2]; z_size + 2];

    for (z, z_layer) in state.iter().enumerate() {
        for (y, y_layer) in z_layer.iter().enumerate() {
            new_state[z + 1][y + 1][1..x_size + 1].copy_from_slice(y_layer);
        }
    }

    for z in 0..new_state.len() {
        let (zmin, zmax) = get_boundaries(z, z_size);

        for y in 0..new_state[z].len() {
            let (ymin, ymax) = get_boundaries(y, y_size);

            for x in 0..new_state[z][y].len() {
                let (xmin, xmax) = get_boundaries(x, x_size);

                let is_active = new_state[z][y][x] == 1;
                let mut active_neighbors = 0u8;
                for z_layer in &state[zmin..zmax] {
                    for y_layer in &z_layer[ymin..ymax] {
                        for &x in &y_layer[xmin..xmax] {
                            active_neighbors += x;
                        }
                    }
                }

                if is_active && active_neighbors != 3 && active_neighbors != 4 {
                    new_state[z][y][x] = 0;
                } else if !is_active && active_neighbors == 3 {
                    new_state[z][y][x] = 1;
                }
            }
        }
    }

    new_state
}

pub fn part_one() -> u64 {
    let mut state = state_3d_from_str(INPUT);

    for _cycle in 0..6 {
        state = run_cycle_part_one(&state);
    }

    state
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|layer| layer.iter().map(|&u| u as u64).sum::<u64>())
                .sum::<u64>()
        })
        .sum::<u64>()
}

fn state_4d_from_str(input: &str) -> State4D {
    let mut state: State4D = vec![];

    state.push(Vec::new());
    state[0].push(Vec::new());
    input.split("\n").enumerate().for_each(|(y, line)| {
        state[0][0].push(Vec::new());
        line.bytes().for_each(|p| {
            if p == b'#' {
                state[0][0][y].push(1)
            } else {
                state[0][0][y].push(0)
            }
        });
    });

    state
}

fn run_cycle_part_two(state: &State4D) -> State4D {
    let w_size = state.len();
    let z_size = state[0].len();
    let y_size = state[0][0].len();
    let x_size = state[0][0][0].len();

    let mut new_state: State4D =
        vec![vec![vec![vec![0; x_size + 2]; y_size + 2]; z_size + 2]; w_size + 2];

    for (w, z_layer) in state.iter().enumerate() {
        for (z, y_layer) in z_layer.iter().enumerate() {
            for (y, x_layer) in y_layer.iter().enumerate() {
                new_state[w + 1][z + 1][y + 1][1..x_size + 1].copy_from_slice(x_layer);
            }
        }
    }

    for w in 0..new_state.len() {
        let (wmin, wmax) = get_boundaries(w, w_size);
        for z in 0..new_state[w].len() {
            let (zmin, zmax) = get_boundaries(z, z_size);
            for y in 0..new_state[w][z].len() {
                let (ymin, ymax) = get_boundaries(y, y_size);
                for x in 0..new_state[w][z][y].len() {
                    let (xmin, xmax) = get_boundaries(x, x_size);

                    let is_active = new_state[w][z][y][x] == 1;
                    let mut active_neighbors = 0u8;
                    for w_layer in &state[wmin..wmax] {
                        for z_layer in &w_layer[zmin..zmax] {
                            for y_layer in &z_layer[ymin..ymax] {
                                for &x in &y_layer[xmin..xmax] {
                                    active_neighbors += x;
                                }
                            }
                        }
                    }

                    if is_active && active_neighbors != 3 && active_neighbors != 4 {
                        new_state[w][z][y][x] = 0;
                    } else if !is_active && active_neighbors == 3 {
                        new_state[w][z][y][x] = 1;
                    }
                }
            }
        }
    }

    new_state
}

pub fn part_two() -> u64 {
    let mut state = state_4d_from_str(INPUT);

    for _cycle in 0..6 {
        state = run_cycle_part_two(&state);
    }

    state
        .iter()
        .map(|layer| {
            layer
                .iter()
                .map(|layer| {
                    layer
                        .iter()
                        .map(|layer| layer.iter().map(|&u| u as u64).sum::<u64>())
                        .sum::<u64>()
                })
                .sum::<u64>()
        })
        .sum::<u64>()
}

pub fn main() {
    let now = Instant::now();
    println!("Day 17: Conway Cubes");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
