extern crate core;

mod day01;
use std::env;
use day01::day01;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        other => println!("Invalid day number {other} as first parameter")
    }
}
