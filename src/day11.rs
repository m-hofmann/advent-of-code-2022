use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::any::Any;
use std::str::{FromStr, Split};
use std::{fmt, fs};
use itertools::Itertools;

struct Monkey {
    items: Box<Vec<u64>>,
    // worry level change upon item inspection
    operation: Box<dyn Fn(&u64) -> u64>,
    // worry level projected to monkey index
    test: Box<dyn Fn(&u64) -> usize>,
    divisor: u64,
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

    println!("starting day 11");

    let contents = fs::read_to_string("data/11_input.txt").expect("Could not read file");
    let lines = contents.split('\n');

    let mut monkeys = parse_input(lines.clone());

    simulate_n_rounds(&mut monkeys, 20, 3);

    let monkey_business_level = monkeys.iter()
        .map(|monkey| monkey.inspected_count)
        .sorted()
        .rev()
        .take(2)
        .inspect(|&it| println!("Inspection count {it}"))
        .fold(1, |acc, elem| acc * elem);

    println!("Part 1: Level of monkey business: {monkey_business_level}");

    monkeys = parse_input(lines.clone());

    simulate_rounds_part2(&mut monkeys, 10_000);

    let monkey_business_level = monkeys.iter()
        .map(|monkey| monkey.inspected_count)
        .sorted()
        .rev()
        .take(2)
        .inspect(|&it| println!("Inspection count {it}"))
        .fold(1 as u128, |acc, elem| acc * elem as u128);

    println!("Part 2: Level of monkey business: {monkey_business_level}");

}

fn parse_input(lines: Split<char>) -> Vec<Monkey> {
    lazy_static! {
        static ref OPERATION_PATTERN: Regex =
            Regex::new(r"Operation: new = old (?P<operator>[\*\+]) (?P<operand>\w+)").unwrap();
    }

    let mut monkeys: Vec<Monkey> = vec![];

    let mut parse_items: Vec<u64> = vec![];
    let mut parse_operation: Box<dyn Fn(&u64) -> u64> = Box::new(|i: &u64| *i);
    // test function is specified on three separate lines
    // to avoid writing a recursive descent parser, we parse each line separately and assemble
    // the function upon creating the Monkey object
    let mut divisor: Option<u64> = None;
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
                    test: Box::new(move |i: &u64| {
                        return if i % divisor.unwrap() == 0 {
                            if_true_monkey.unwrap()
                        } else {
                            if_false_monkey.unwrap()
                        };
                    }),
                    divisor: divisor.unwrap(),
                    inspected_count: 0,
                });
                // avoid moving same Fn twice
                parse_operation = Box::new(|i: &u64| *i);
            }
        } else if line.starts_with("Monkey") {
            // skip, we do counting implicitly
        } else if line.trim().starts_with("Starting items: ") {
            parse_items = line
                .trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|it| u64::from_str(it).unwrap())
                .collect::<Vec<u64>>();
        } else if line.trim().starts_with("Operation: ") {
            OPERATION_PATTERN
                .captures(line.trim())
                .and_then::<Captures, _>(|cap| {
                    parse_operation = match cap.name("operator").unwrap().as_str() {
                        "+" => match cap.name("operand").unwrap().as_str() {
                            "old" => Box::new(|i: &u64| i + i),
                            other => {
                                let tmp = u64::from_str(other).unwrap();
                                Box::new(move |i: &u64| i + &tmp)
                            }
                        },
                        "*" => match cap.name("operand").unwrap().as_str() {
                            "old" => Box::new(|i: &u64| i * i),
                            other => {
                                let tmp = u64::from_str(other).unwrap();
                                Box::new(move |i: &u64| i * &tmp)
                            }
                        },
                        other => panic!("Unknown operator {other}"),
                    };
                    return None;
                });
        } else if line.trim().starts_with("Test: divisible by") {
            divisor = u64::from_str(line.trim().strip_prefix("Test: divisible by ").unwrap()).ok()
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
    return monkeys;
}

fn simulate_n_rounds(monkeys: &mut Vec<Monkey>, n: u32, divisor: u64) {
    for _ in 1..=n {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let throws: Vec<(usize, u64)> = monkey.items.iter()
                .map(|item| {
                    monkey.inspected_count += 1;
                    let new_worry: u64 = (monkey.operation)(item)/divisor;
                    let target_monkey: usize = (monkey.test)(&new_worry);
                    (target_monkey, new_worry)
                })
                .collect::<Vec<(usize, u64)>>();
            monkey.items = Box::new(vec![]);
            for (target, item) in throws {
                monkeys[target].items.push(item);
            }
        }
    }
}

fn simulate_rounds_part2(monkeys: &mut Vec<Monkey>, n: u32) {
    let divisor_factor: u64 = monkeys.iter().map(|it| it.divisor).inspect(|it| println!("Divisor {it}")).product();
    for _ in 1..=n {
        for i in 0..monkeys.len() {
            let monkey = monkeys.get_mut(i).unwrap();
            let throws: Vec<(usize, u64)> = monkey.items.iter()
                .map(|item| {
                    monkey.inspected_count += 1;
                    // under the assumption that all test integers are prime, storing remainder is good enough
                    let new_worry: u64 = (monkey.operation)(item) % divisor_factor;
                    let target_monkey: usize = (monkey.test)(&new_worry);
                    (target_monkey, new_worry)
                })
                .collect::<Vec<(usize, u64)>>();
            monkey.items = Box::new(vec![]);
            for (target, item) in throws {
                monkeys[target].items.push(item);
            }
        }
    }
}
