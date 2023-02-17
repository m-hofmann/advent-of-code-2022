use std::collections::{HashSet};
use std::fs;
use std::str::FromStr;

pub fn day08() {
    println!("starting day 08");

    let contents =
        fs::read_to_string("data/08_demo.txt").expect("Could not read file");

    let lines = contents.split('\n');

    let mut grid : Vec<Vec<u8>> = vec![];
    for line_raw in lines {
        let mut line_conv = vec![];
        for i in 0..line_raw.len() {
            line_conv.push(u8::from_str(&line_raw[i..i+1]).unwrap())
        }
        grid.push(line_conv);
    }

    println!("Grid: \n{:?}", grid);
}

