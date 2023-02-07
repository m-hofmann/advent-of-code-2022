use std::fs;
use std::str::FromStr;
use regex::{Captures, Regex};
use lazy_static::lazy_static;

fn extract_group_to_u32(cap: &Captures, name: &str) -> u32 {
    return cap.name(name).map(|it| u32::from_str(it.as_str()).unwrap()).unwrap();
}

pub fn day04() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<start1>\d+)-(?P<end1>\d+),(?P<start2>\d+)-(?P<end2>\d+)").unwrap();
    }
    println!("starting day 04");

    let contents = fs::read_to_string("data/04_1_overlapping_assignments.txt")
        .expect("Could not read file");

    let mut overlapping_section_pairs = 0;
    for line in contents.split('\n') {
        RE.captures(line).and_then::<Captures, _>(|cap| {
            let s1 = extract_group_to_u32(&cap, "start1");
            let e1 = extract_group_to_u32(&cap, "end1");
            let s2 = extract_group_to_u32(&cap, "start2");
            let e2 = extract_group_to_u32(&cap, "end2");

            if (s1 >= s2 && e1 >= s2 && e1 <= e2) || (s2 >= s1  && s2 <= e2 && e2 <= e1) {
                overlapping_section_pairs += 1;
            }
            return None;
        });
    }
    println!("Pairs of elves with overlapping sections {overlapping_section_pairs}");
}