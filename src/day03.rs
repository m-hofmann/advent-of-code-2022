use std::collections::HashSet;
use std::fs;

fn char_to_value(value: char) -> u32 {
    return match value  {
        'a'..='z' => value as u32 - 'a' as u32 + 1,
        'A'..='Z' => value as u32 - 'A' as u32 + 27,
        _other => 0
    }
}

pub fn day03() {
    println!("starting day 03");

    let contents = fs::read_to_string("data/03_1_rucksack.txt")
        .expect("Could not read file");

    let mut accu = 0;
    for line in contents.split('\n') {
        let half = line.len()/2;
        let first_as_set : HashSet<char> = HashSet::from_iter(line[0..half].chars().into_iter());
        let second_as_set : HashSet<char> = HashSet::from_iter(line[half..line.len()].chars().into_iter());

        let intersection = first_as_set.intersection(&second_as_set).next().unwrap();

        accu += char_to_value(*intersection);
    }

    println!("value of all duplicate rucksack items {accu}")

}