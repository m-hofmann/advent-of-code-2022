use std::fs;
use std::str::Split;
use itertools::Itertools;
use crate::day13::Token::{List, Num};


#[derive(Debug)]
enum Token {
    Num(u32),
    List(Box<Vec<Token>>)
}

fn parse_list(s: &mut String) -> Vec<Token> {
    let mut stack = vec![];
    let mut buf = String::new();

    while let Some(c) = s.pop() {
        match c {
            '[' => stack.push(List(Box::new(parse_list(s)))),
            ']' => {
                match buf.parse() {
                    Ok(num) => stack.push(Num(num)),
                    _ => {}
                }
                return stack;
            },
            ',' => {
                if let Ok(number) = buf.parse::<u32>() {
                    stack.push(Num(number));
                    buf = String::new();
                }
            },
            c => {
                buf.push(c);
            },
            other => panic!("Cannot parse char {}", other),
        }
    }

    return stack;
}

pub fn day13() {
    println!("starting day 11");

    let contents = fs::read_to_string("data/13_demo.txt").expect("Could not read file");
    let lines = contents.split('\n');

    let mut pair_cnt = 1;
    let mut correct_indices_count = 0;
    for (left_raw, right_raw) in lines.filter(|&it| !it.is_empty()).tuples::<(&str, &str)>() {
        let mut left_rev = String::from(left_raw).chars().rev().collect();
        let mut right_rev = String::from(right_raw).chars().rev().collect();
        let left = parse_list(&mut left_rev);
        let right = parse_list(&mut right_rev);


        println!("left {:?}", left_raw);
        println!("left (parsed) {:?}", left);
        println!("right {:?}", right_raw);
        println!("right (parsed) {:?}", right);

        pair_cnt += 1;
    }

    println!("Sum of indices of correct pairs {}", correct_indices_count);
}
