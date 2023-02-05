use std::fs;

pub fn day01() {
    println!("starting day01");

    let contents = fs::read_to_string("data/01_elves_calories.txt")
        .expect("Could not read file");

    let mut calories_by_elf = Vec::new();
    let mut accu = 0;

    for line in contents.split('\n') {
        match line {
            _ if line.is_empty() => {
                calories_by_elf.push(accu);
                accu = 0;
            }
            _ => {
                let calories: i32 = line.trim().parse().expect("Must be a number");
                accu += calories;
            }
        }
    }

    let max = calories_by_elf.iter().max();
    match max {
        Some(value) => println!("Maximum calories carried by one elf {value}"),
        None => println!("Could not determine calories by elf")
    }

    calories_by_elf.sort_unstable();
    let top3 : i32 = calories_by_elf.iter().rev().take(3).sum();
    println!("Top3 elves {:?}", top3)
}