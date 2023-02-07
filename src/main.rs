extern crate core;

mod day01;
mod day02;
mod day03;
mod day04;
use std::env;
use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        "02" => day02(),
        "03" => day03(),
        "04" => day04(),
        other => println!("Invalid day number {other} as first parameter")
    }
}
