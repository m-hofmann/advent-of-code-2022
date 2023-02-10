use std::collections::{HashSet};
use std::fs;


pub fn day06() {
    println!("starting day 06");

    let contents =
        fs::read_to_string("data/06_1_signal_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    for line in lines {
        println!("Part 1: Signal start marker: {:?}", get_signal_start_pos(line.clone(), 4));
        println!("Part 2: Message start marker: {:?}", get_signal_start_pos(line.clone(), 14));
    }
}

fn get_signal_start_pos(input: &str, distinct_chunk_size: usize) -> usize {
    if input.len() < distinct_chunk_size {
        panic!("Input string too short.");
    }

    let chars = input.chars().collect::<Vec<char>>();
    for i in (distinct_chunk_size)..chars.len() {
        let set : HashSet<char> = chars[(i-distinct_chunk_size)..i].iter()
            .map(|x|*x)
            .collect();
        if set.len() == distinct_chunk_size {
            return i;
        }
    }
    return 0;
}
