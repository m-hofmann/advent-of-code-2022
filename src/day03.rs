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

    println!("value of all duplicate rucksack items {accu}");

    let mut accu_part2 = 0;

    for chunk in contents.split('\n')
        .collect::<Vec<&str>>()
        .chunks(3) {
        chunk[0].chars().collect::<HashSet<char>>();
        let a = chunk[0].chars().collect::<HashSet<char>>();
        let b = chunk[1].chars().collect::<HashSet<char>>();
        let c = chunk[2].chars().collect::<HashSet<char>>();

        let mut two_elves_intersection = a.intersection(&b)
            .map(|it| *it)
            .collect::<HashSet<char>>();
        let mut three_elves_intersection = two_elves_intersection.intersection(&c);

        accu_part2 += char_to_value(*three_elves_intersection.next().unwrap());
    }

    println!("value of all badges for three-tuples of elves {accu_part2}")

}