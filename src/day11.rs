use std::{fmt, fs};
use std::any::Any;
use std::str::{FromStr};
use regex::{Captures, Regex};
use lazy_static::lazy_static;


struct Monkey {
    items: Vec<u32>,
    // worry level change upon item inspection
    operation: Box<dyn Fn(u32) -> u32>,
    // worry level projected to monkey index
    test: Box<dyn Fn(u32) -> usize>
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("operation", &self.operation.type_id())
            .field("test", &self.operation.type_id())
            .finish()
    }
}

pub fn day11() {
    lazy_static! {
    static ref OPERATION_PATTERN: Regex =
        Regex::new(r"Operation: old (?P<operator>[\*\+]) (?P<operand>\w+)").unwrap();
    }
    println!("starting day 10");

    let contents =
        fs::read_to_string("data/11_demo.txt").expect("Could not read file");

    let lines = contents.split('\n');

    let mut monkeys : Vec<Monkey> = vec![];
    let mut parse_items : Vec<u32> = vec![];
    let mut parse_operation : Box<dyn Fn(u32) -> u32> = Box::new(|i: u32| i);
    // test function is specified on three separate lines
    // to avoid writing a recursive descent parser, we parse each line separately and assemble
    // the function upon creating the Monkey object
    let mut divisor : Option<u32> = None;
    let mut if_true_monkey : Option<usize> = None;
    let mut if_false_monkey : Option<usize> = None;


    for line in lines {
        if line.is_empty() {
            {
                if divisor.is_none()
                    || if_true_monkey.is_none()
                    || if_false_monkey.is_none() {
                    panic!("Not all necessary parameters for a test function have been assembled, yet a Monkey should be build.")
                }
                monkeys.push(Monkey {
                    items: parse_items.clone(),
                    operation: parse_operation,
                    test: Box::new(move |i: u32| {
                        return if i % divisor.unwrap() == 0 {
                            if_true_monkey.unwrap()
                        } else {
                            if_false_monkey.unwrap()
                        }

                    })
                });
                // avoid moving same Fn twice
                parse_operation = Box::new(|i: u32| i);
            }
        } else if line.starts_with("Monkey") {
            // skip, we do counting implicitly
        } else if line.trim().starts_with("Starting items: ") {
            parse_items = line.trim().strip_prefix("Starting items: ").unwrap().split(", ")
                .map(|it| u32::from_str(it).unwrap())
                .collect::<Vec<u32>>();
        } else if line.trim().starts_with("Operation: ") {
            OPERATION_PATTERN.captures(line.trim()).and_then::<Captures, _>(|cap| {
                parse_operation =
                    match cap.name("operator").unwrap().as_str() {
                        "+" =>
                        match cap.name("operand").unwrap().as_str() {
                            "old" => Box::new(|i: u32| i + i),
                            other => {
                                let tmp = u32::from_str(other).unwrap();
                                Box::new(move |i: u32| { i + &tmp })
                            },
                        },
                        "*" => match cap.name("operand").unwrap().as_str() {
                            "old" => Box::new(|i: u32| i * i),
                            other => {
                                let tmp = u32::from_str(other).unwrap();
                                Box::new(move |i: u32| { i * &tmp })
                            },
                        },
                        other => panic!("Unknown operator {other}")
                    };
                return None;
            });
        } else if line.trim().starts_with("Test: divisible by") {
            divisor = u32::from_str(line.trim().strip_prefix("Test: divisible by ").unwrap()).ok()
        } else if line.trim().starts_with("If true: throw to monkey ") {
            if_true_monkey = usize::from_str(line.trim().strip_prefix("If true: throw to monkey ").unwrap()).ok()
        } else if line.trim().starts_with("If false: throw to monkey ") {
            if_false_monkey = usize::from_str(line.trim().strip_prefix("If false: throw to monkey ").unwrap()).ok()
        } else {
            panic!("Cannot parse line {line}")
        }
    }

    println!("Parsed monkeys: {:?}", monkeys)


}