use std::collections::{HashSet};
use std::fs;


pub fn day06() {
    println!("starting day 06");

    let contents =
        fs::read_to_string("data/06_1_signal_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    for line in lines {
        println!("Signal start marker: {:?}", get_signal_start_pos(line));
    }
}

fn get_signal_start_pos(input: &str) -> usize {
    if input.len() < 4 {
        panic!("Input string too short.");
    }

    let chars = input.chars().collect::<Vec<char>>();
    for i in 3..chars.len() {
        let mut set : HashSet<char> = HashSet::new();
        set.insert(chars[i-3]);
        set.insert(chars[i-2]);
        set.insert(chars[i-1]);
        set.insert(chars[i]);

        if set.len() == 4 {
            return i + 1;
        }
    }
    return 0;
}
