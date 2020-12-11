mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;
mod day_08;
mod day_09;
mod day_10;
mod day_11;

use std::time::Instant;

fn main() {
    let now = Instant::now();
    day_01::main();
    day_02::main();
    day_03::main();
    day_04::main();
    day_05::main();
    day_06::main();
    day_07::main();
    day_08::main();
    day_09::main();
    day_10::main();
    day_11::main();
    println!("Total time: {}ms", now.elapsed().as_millis());
}
