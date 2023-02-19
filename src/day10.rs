use std::fs;
use std::str::{FromStr};

pub fn day10() {

    println!("starting day 10");

    let contents =
        fs::read_to_string("data/10_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    let mut cycle = 0;
    let mut x = 1;
    let mut cycle_x : Vec<i32> = vec![x];
    for line in lines {
        if line.starts_with("noop") {
            cycle_x.push(x);
            cycle = cycle + 1;
        } else if line.starts_with("addx ") {
            let op = line.strip_prefix("addx ").and_then(|it| i32::from_str(it).ok()).unwrap();
            cycle_x.push(x);
            cycle = cycle + 1;
            cycle_x.push(x);
            x = x + op;
            cycle = cycle + 1;
        }
        //println!("after cycle {cycle} value {x} (command was {line})")
    }

    let interesting = vec![20, 60, 100, 140, 180, 220];
    let cycle_sum : i32 = cycle_x.iter().enumerate()
        .filter(|(i, _)| interesting.contains(i))
        .map(|(i, &x)| i as i32 * x)
        .sum();
    println!("Sum of signal strengths is {:?}", cycle_sum)

}