mod day_01;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day_01::main();
    println!("Total time: {}µs", now.elapsed().as_micros());
}
