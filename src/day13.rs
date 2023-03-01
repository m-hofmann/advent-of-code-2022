use std::fs;
use std::str::Split;
use itertools::Itertools;

fn compare_stack_based(left: &mut Vec<char>, right: &mut Vec<char>) -> bool {

    while !left.is_empty() && !right.is_empty() {
        let l = left.pop().unwrap();
        let r = right.pop().unwrap();

        if l == '[' && r.is_numeric() {
            right.push(']');
            right.push(r);
        } else if r == '[' && l.is_numeric() {
            left.push(']');
            left.push(l);
        } else if l.is_numeric() && r.is_numeric() {
            let l_u32 = l.to_digit(10).unwrap();
            let r_u32 = r.to_digit(10).unwrap();
            //println!("  - Compare {} vs {}", l_u32, r_u32);
            if l_u32 > r_u32 {
                //println!("    - Right side is smaller, so inputs are not in correct order.");
                return false;
            } else if l_u32 < r_u32 {
                //println!("    - Left side is smaller, so inputs are in correct order.");
                return true;
            }
        } else if l == ',' && l == r {
            // pass, comma
        } else if l == '[' && l == r {
            // pass, list start
        } else if l == ']' && r != ']' {
            // left is smaller
            return true;
        } else {
            //println!("Ignoring l {}, r {}", l, r);
        }
    }
    if left.is_empty() {
        return true;
    } else {
        return false;
    }
}

pub fn day13() {
    println!("starting day 11");

    let contents = fs::read_to_string("data/13_input.txt").expect("Could not read file");
    let lines = contents.split('\n');

    let mut pair_cnt = 1;
    let mut correct_indices_count = 0;
    for (left_raw, right_raw) in lines.filter(|&it| !it.is_empty()).tuples::<(&str, &str)>() {
        let mut left = String::from(left_raw).chars().rev().collect::<Vec<char>>();
        let mut right = String::from(right_raw).chars().rev().collect::<Vec<char>>();

        println!("== Pair {} == ", pair_cnt);
        println!("- Compare {} vs {}", left_raw, right_raw);

        let result = compare_stack_based(&mut left, &mut right);
        match result {
            true => {
                println!("Correct order");
                correct_indices_count += pair_cnt;
            },
            false => println!("Incorrect order")
        }

        pair_cnt += 1;
    }

    println!("Sum of indices of correct pairs {}", correct_indices_count);
}
