extern crate core;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
use std::env;
use day01::day01;
use day02::day02;
use day03::day03;
use day04::day04;
use day05::day05;
use day06::day06;
use day07::day07;
use day08::day08;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "01" => day01(),
        "02" => day02(),
        "03" => day03(),
        "04" => day04(),
        "05" => day05(),
        "06" => day06(),
        "07" => day07(),
        "08" => day08(),
        other => println!("Invalid day number {other} as first parameter")
    }
}
