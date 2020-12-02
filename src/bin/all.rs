mod day_01;
mod day_02;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day_01::main();
    day_02::main();
    println!("Total time: {}ms", now.elapsed().as_millis());
}
