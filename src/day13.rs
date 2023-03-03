use crate::day13::Token::{List, Num};
use itertools::Itertools;
use std::fs;
use std::str::Split;

#[derive(Debug)]
enum Token {
    Num(u32),
    List(Box<Vec<Token>>),
}

fn parse_input(s: &String) -> Token {
    // reverse string as parser works with Vec::pop, which consumes Vec from end
    let mut rev = s.chars().rev().collect::<String>();
    let result = match rev.pop() {
        Some('[') => parse_list(&mut rev),
        other => panic!("Illegal input - does not start with [ but with {:?}", other),
    };

    return result;
}

fn parse_list(s: &mut String) -> Token {
    let mut stack = vec![];
    let mut buf = String::new();

    while let Some(c) = s.pop() {
        match c {
            '[' => stack.push(parse_list(s)),
            ']' => {
                match buf.parse() {
                    Ok(num) => stack.push(Num(num)),
                    _ => {}
                }
                return List(Box::new(stack));
            }
            ',' => {
                if let Ok(number) = buf.parse::<u32>() {
                    stack.push(Num(number));
                    buf = String::new();
                }
            }
            c => {
                buf.push(c);
            }
            other => panic!("Cannot parse char {}", other),
        }
    }

    return List(Box::new(stack));
}

fn compare(left: &Token, right: &Token) -> Option<bool> {
    return match (left, right) {
        (List(left_list), List(right_list)) => {
            let mut li = left_list.iter();
            let mut ri = right_list.iter();

            loop {
                match (li.next(), ri.next()) {
                    (Some(l), Some(r)) => match compare(l, r) {
                        Some(true) => return Some(true),
                        Some(false) => return Some(false),
                        None => {},
                    },
                    (None, Some(_)) => return Some(true),
                    (Some(_), None) => return Some(false),
                    (None, None) => return None,
                }
            }
        }
        (List(_), Num(r)) => compare(left, &List(Box::new(vec![Num(*r)]))),
        (Num(l), List(_)) => compare(&List(Box::new(vec![Num(*l)])), right),
        (Num(l), Num(r)) => {
            if l < r {
                Some(true)
            } else if l == r {
                None
            } else {
                Some(false)
            }
        }
    };
}

pub fn day13() {
    println!("starting day 11");

    let contents = fs::read_to_string("data/13_input.txt").expect("Could not read file");
    let lines = contents.split('\n');

    let mut pair_cnt = 1;
    let mut correct_indices_count = 0;
    for (left_raw, right_raw) in lines.filter(|&it| !it.is_empty()).tuples::<(&str, &str)>() {
        let left = parse_input(&String::from(left_raw));
        let right = parse_input(&String::from(right_raw));
        println!("== Pair {} ==", pair_cnt);

        //println!("left {:?}", left_raw);
        //println!("left (parsed) {:?}", left);
        //bprintln!("right {:?}", right_raw);
        //println!("right (parsed) {:?}", right);
        let res = compare(&left, &right);
        match res {
            Some(true) => {
                println!("Are in right order");
                correct_indices_count += pair_cnt;
            }
            Some(false) => println!("Not in right order"),
            None => panic!("Comparison failed"),
        }

        pair_cnt += 1;
    }

    println!("Sum of indices of correct pairs {}", correct_indices_count);
}
