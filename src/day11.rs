use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::any::Any;
use std::str::FromStr;
use std::{fmt, fs};
use ibig::{UBig, ubig};
use itertools::Itertools;

struct Monkey {
    items: Box<Vec<UBig>>,
    // worry level change upon item inspection
    operation: Box<dyn Fn(&UBig) -> UBig>,
    // worry level projected to monkey index
    test: Box<dyn Fn(&UBig) -> usize>,
    inspected_count : u128,
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
            Regex::new(r"Operation: new = old (?P<operator>[\*\+]) (?P<operand>\w+)").unwrap();
    }
    println!("starting day 11");

    let contents = fs::read_to_string("data/11_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    let mut monkeys: Vec<Monkey> = vec![];
    let mut parse_items: Vec<UBig> = vec![];
    let mut parse_operation: Box<dyn Fn(&UBig) -> UBig> = Box::new(|i: &UBig| i.clone());
    // test function is specified on three separate lines
    // to avoid writing a recursive descent parser, we parse each line separately and assemble
    // the function upon creating the Monkey object
    let mut divisor: Option<u32> = None;
    let mut if_true_monkey: Option<usize> = None;
    let mut if_false_monkey: Option<usize> = None;

    for line in lines {
        if line.is_empty() {
            {
                if divisor.is_none() || if_true_monkey.is_none() || if_false_monkey.is_none() {
                    panic!("Not all necessary parameters for a test function have been assembled, yet a Monkey should be build.")
                }
                monkeys.push(Monkey {
                    items: Box::new(parse_items.clone()),
                    operation: parse_operation,
                    test: Box::new(move |i: &UBig| {
                        return if i % divisor.unwrap() == 0 {
                            if_true_monkey.unwrap()
                        } else {
                            if_false_monkey.unwrap()
                        };
                    }),
                    inspected_count: 0,
                });
                // avoid moving same Fn twice
                parse_operation = Box::new(|i: &UBig| i.clone());
            }
        } else if line.starts_with("Monkey") {
            // skip, we do counting implicitly
        } else if line.trim().starts_with("Starting items: ") {
            parse_items = line
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|it| UBig::from_str(it).unwrap())
                .collect::<Vec<UBig>>();
        } else if line.trim().starts_with("Operation: ") {
            OPERATION_PATTERN
                .captures(line.trim())
                .and_then::<Captures, _>(|cap| {
                    parse_operation = match cap.name("operator").unwrap().as_str() {
                        "+" => match cap.name("operand").unwrap().as_str() {
                            "old" => Box::new(|i: &UBig| i + i),
                            other => {
                                let tmp = UBig::from_str(other).unwrap();
                                Box::new(move |i: &UBig| i + &tmp)
                            }
                        },
                        "*" => match cap.name("operand").unwrap().as_str() {
                            "old" => Box::new(|i: &UBig| i * i),
                            other => {
                                let tmp = UBig::from_str(other).unwrap();
                                Box::new(move |i: &UBig| i * &tmp)
                            }
                        },
                        other => panic!("Unknown operator {other}"),
                    };
                    return None;
                });
        } else if line.trim().starts_with("Test: divisible by") {
            divisor = u32::from_str(line.trim().strip_prefix("Test: divisible by ").unwrap()).ok()
        } else if line.trim().starts_with("If true: throw to monkey ") {
            if_true_monkey = usize::from_str(
                line.trim()
                    .strip_prefix("If true: throw to monkey ")
                    .unwrap(),
            )
            .ok()
        } else if line.trim().starts_with("If false: throw to monkey ") {
            if_false_monkey = usize::from_str(
                line.trim()
                    .strip_prefix("If false: throw to monkey ")
                    .unwrap(),
            )
            .ok()
        } else {
            panic!("Cannot parse line {line}")
        }
    }

    println!("Parsed {:?} monkeys: {:?}", monkeys.len(), monkeys);

    simulate_n_rounds(&mut monkeys, 20, &ubig!(3));

    let monkey_business_level = monkeys.iter()
        .map(|monkey| monkey.inspected_count)
        .sorted()
        .rev()
        .take(2)
        .inspect(|&it| println!("Inspection count {it}"))
        .fold(1, |acc, elem| acc * elem);

    println!("Part 1: Level of monkey business: {monkey_business_level}");


    // as monkeys are not cloneable, we cannot use clones of the monkey vector
    // for each part of the riddle
    // therefore, reset inspected count here
    for i in 0..monkeys.len() {
        monkeys[i].inspected_count = 0;
    }

    simulate_n_rounds(&mut monkeys, 1000, &ubig!(1));

    let monkey_business_level = monkeys.iter()
        .map(|monkey| monkey.inspected_count)
        .sorted()
        .rev()
        .take(2)
        .inspect(|&it| println!("Inspection count {it}"))
        .fold(1 as u128, |acc, elem| acc * elem as u128);

    println!("Part 2: Level of monkey business: {monkey_business_level}");

}

fn simulate_n_rounds(monkeys: &mut Vec<Monkey>, n: u32, divisor: &UBig) {
    for _ in 1..=n {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let throws: Vec<(usize, UBig)> = monkey.items.iter()
                .map(|item| {
                    monkey.inspected_count += 1;
                    let new_worry: UBig = (monkey.operation)(item)/divisor;
                    let target_monkey: usize = (monkey.test)(&new_worry);
                    (target_monkey, new_worry)
                })
                .collect::<Vec<(usize, UBig)>>();
            monkey.items = Box::new(vec![]);
            for (target, item) in throws {
                monkeys[target].items.push(item);
            }
        }
    }
}
