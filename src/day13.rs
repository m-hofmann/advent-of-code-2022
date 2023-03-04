use crate::day13::Token::{List, Num};
use itertools::Itertools;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fs;

#[derive(Debug, Ord, Eq, PartialEq, Clone)]
enum Token {
    Num(u32),
    List(Box<Vec<Token>>),
}

impl PartialOrd for Token {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return match (self, other) {
            (List(left_list), List(right_list)) => {
                let mut li = left_list.iter();
                let mut ri = right_list.iter();

                loop {
                    match (li.next(), ri.next()) {
                        (Some(l), Some(r)) => match l.partial_cmp(r) {
                            Some(ordering) => return Some(ordering),
                            None => {}
                        },
                        (None, Some(_)) => return Some(Less),
                        (Some(_), None) => return Some(Greater),
                        (None, None) => return None,
                    }
                }
            }
            (List(_), Num(r)) => self.partial_cmp(&List(Box::new(vec![Num(*r)]))),
            (Num(l), List(_)) => List(Box::new(vec![Num(*l)])).partial_cmp(&other),
            (Num(l), Num(r)) => {
                if l < r {
                    Some(Less)
                } else if l == r {
                    None
                } else {
                    Some(Greater)
                }
            }
        };
    }
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

pub fn day13() {
    println!("starting day 11");

    let contents = fs::read_to_string("data/13_demo.txt").expect("Could not read file");
    let lines = contents.split('\n');

    let mut pair_cnt = 1;
    let mut correct_indices_count = 0;
    for (left_raw, right_raw) in lines
        .clone()
        .filter(|&it| !it.is_empty())
        .tuples::<(&str, &str)>()
    {
        let left = parse_input(&String::from(left_raw));
        let right = parse_input(&String::from(right_raw));
        println!("== Pair {} ==", pair_cnt);

        //println!("left {:?}", left_raw);
        //println!("left (parsed) {:?}", left);
        //bprintln!("right {:?}", right_raw);
        //println!("right (parsed) {:?}", right);
        let res = &left.partial_cmp(&right);
        match res {
            Some(cmp) => match cmp {
                x @ Less | x @ Ordering::Equal => {
                    println!("<: Are in correct order");
                    correct_indices_count += pair_cnt;
                }
                Greater => println!(">: Not in correct order"),
            },
            None => panic!("Comparison failed"),
        }

        pair_cnt += 1;
    }

    println!(
        "Part 1: Sum of indices of correct pairs {}",
        correct_indices_count
    );

    let divider_2 = List(Box::new(vec![List(Box::new(vec![Num(2)]))]));
    let divider_6 = List(Box::new(vec![List(Box::new(vec![Num(6)]))]));
    let mut lists = vec![divider_2.clone(), divider_6.clone()];

    for line in lines.clone().filter(|&l| !l.is_empty()) {
        lists.push(parse_input(&mut String::from(line)));
    }

    lists.sort();

}
