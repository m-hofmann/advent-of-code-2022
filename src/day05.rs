use itertools::Itertools;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;
use std::str::{FromStr, Split};

fn capture_group_to_u32(cap: &Captures, name: &str) -> u32 {
    return cap
        .name(name)
        .map(|it| u32::from_str(it.as_str()).unwrap())
        .unwrap();
}
lazy_static! {
    static ref RE: Regex =
        Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();
}

pub fn day05() {
    println!("starting day 05");

    let contents =
        fs::read_to_string("data/05_1_stack_rearrangement.txt").expect("Could not read file");

    let mut lines = contents.split('\n');
    // use vec as stacks: "front" is "bottom", "end" is "top"
    let stacks: HashMap<usize, Vec<char>> = parse_input(&mut lines);
    let mut stack_after_part1 = part1_cratemover9000(&mut lines.clone(), stacks.clone());
    print_top_crates(&mut stack_after_part1);
    let mut stack_after_part2 = part2_cratemover9001(&mut lines.clone(), stacks.clone());
    print_top_crates(&mut stack_after_part2);
}

fn print_top_crates(stack_after_part1: &mut HashMap<usize, Vec<char>>) {
    let top_crates_part1 = stack_after_part1
        .iter_mut()
        .sorted_by_key(|x| x.0)
        .map(|(_, v)| v.last().unwrap_or(&' '))
        .collect::<String>();
    println!("Top crates: {top_crates_part1}");
}

fn parse_input(lines: &mut Split<char>) -> HashMap<usize, Vec<char>> {
    let mut stacks = HashMap::new();
    // parse initial stack setup
    // iterate over lines and take every 4th char
    for line in lines {
        for (i, c) in line.chars().enumerate().skip(1).step_by(4) {
            let stack_index = i / 4;
            // set up initial stack container if not already present
            if !stacks.contains_key(&stack_index) {
                stacks.insert(stack_index, Vec::new());
            }

            if c.is_uppercase() {
                stacks.get_mut(&stack_index).unwrap().push(c);
            }
        }
        if line.is_empty() {
            // reverse all stacks as they have been inserted in the wrong order
            // (file goes "top-to-bottom" while we always append to the stack)
            for (_, stack) in stacks.iter_mut() {
                stack.reverse()
            }
            break;
        }
    }
    return stacks;
}

fn part1_cratemover9000(lines: &mut Split<char>, mut stacks: HashMap<usize, Vec<char>>) -> HashMap<usize, Vec<char>> {
    for line in lines {
        RE.captures(line).and_then::<Captures, _>(|cap| {
            let count = capture_group_to_u32(&cap, "count");
            let from_index = capture_group_to_u32(&cap, "from") as usize - 1;
            let to_index = capture_group_to_u32(&cap, "to") as usize - 1;

            for _ in 0..count {
                let moved_crate = stacks
                    .get_mut(&from_index)
                    .unwrap_or_else(|| panic!("No stack with from index {from_index}"))
                    .pop()
                    .unwrap_or_else(|| panic!("Trying to pop from empty stack #{from_index} "));
                stacks
                    .get_mut(&to_index)
                    .unwrap_or_else(|| panic!("Could not access stack {to_index} to push to"))
                    .push(moved_crate);
            }

            return None;
        });
    }
    return stacks;

}

fn part2_cratemover9001(lines: &mut Split<char>, mut stacks: HashMap<usize, Vec<char>>) -> HashMap<usize, Vec<char>> {
    for line in lines {
        RE.captures(line).and_then::<Captures, _>(|cap| {
            let count = capture_group_to_u32(&cap, "count") as usize;
            let from_index = capture_group_to_u32(&cap, "from") as usize - 1;
            let to_index = capture_group_to_u32(&cap, "to") as usize - 1;
            let source_stack = stacks
                    .get_mut(&from_index)
                    .unwrap_or_else(|| panic!("No stack with from index {from_index}"));
            let mut moved_crates = source_stack.split_off(source_stack.len() - count);

            stacks.get_mut(&to_index)
                .unwrap_or_else(|| panic!("Could not access stack {to_index} to push to"))
                .append(&mut moved_crates);

            return None;
        });
    }
    return stacks;
}
