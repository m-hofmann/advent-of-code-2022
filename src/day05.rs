use std::collections::{HashMap};
use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use regex::{Captures, Regex};
use lazy_static::lazy_static;

fn capture_group_to_u32(cap: &Captures, name: &str) -> u32 {
    return cap.name(name).map(|it| u32::from_str(it.as_str()).unwrap()).unwrap();
}

pub fn day05() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
    }
    println!("starting day 05");

    let contents = fs::read_to_string("data/05_1_stack_rearrangement.txt")
        .expect("Could not read file");
        // parse input
    // use vec as stacks: "front" is "bottom", "end" is "top"
    let mut stacks : HashMap<usize, Vec<char>> = HashMap::new();

    let mut is_stack_input = true;
    for line in contents.split('\n') {
        if is_stack_input {
            // parse initial stack setup
            // iterate over lines and take every 4th char
            for (i, c) in line.chars().enumerate()
                .skip(1)
                .step_by(4) {
                let stack_index = i/4;
                // set up initial stack container if not already present
                if !stacks.contains_key(&stack_index) {
                    stacks.insert(stack_index, Vec::new());
                }

                if c.is_uppercase() {
                    stacks.get_mut(&stack_index).unwrap().push(c);
                }
            }
            if line.is_empty() {
                is_stack_input = false;
                // reverse all stacks as they have been inserted in the wrong order
                // (file goes "top-to-bottom" while we always append to the stack)
                for (_, stack) in stacks.iter_mut() {
                    stack.reverse()
                }
            }
        } else {
            RE.captures(line).and_then::<Captures, _>(|cap| {
                let count = capture_group_to_u32(&cap, "count");
                let from_index = capture_group_to_u32(&cap, "from") as usize - 1;
                let to_index = capture_group_to_u32(&cap, "to") as usize - 1;

                for _ in 0..count {
                    let moved_crate = stacks.get_mut(&from_index)
                        .unwrap_or_else(|| panic!("No stack with from index {from_index}"))
                        .pop()
                        .unwrap_or_else(|| panic!("Trying to pop from empty stack #{from_index} "));
                    stacks.get_mut(&to_index)
                        .unwrap_or_else(|| panic!("Could not access stack {to_index} to push to"))
                        .push(moved_crate);
                }

                return None;
            });
        }
    }

    let top_crates = stacks.iter_mut()
        .sorted_by_key(|x| x.0)
        .map(|(_, v)| v.last().unwrap_or(&' '))
        .collect::<String>();
    println!("Top crates: {top_crates}");

}