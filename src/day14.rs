use lazy_static::lazy_static;
use regex::{Regex};
use std::collections::HashMap;
use std::fs;
use std::str::Split;

#[derive(Debug, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl PartialEq<Self> for Coord {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Coord {

}

#[derive(Debug)]
enum Material {
    Air,
    Rock,
    Sand,
    Source,
}

// see https://stackoverflow.com/a/70352626
fn range_inclusive_updown(a: usize, b: usize) -> impl Iterator<Item = usize> {
    let x: Box<dyn Iterator<Item = usize>>;
    if b > a {
        x = Box::new(a..=b);
    } else {
        x = Box::new(b..=a)
    }
    x
}

fn print_cave(cave: HashMap<Coord, Material>) {
    let mut min_x = usize::MAX;
    let mut max_x = usize::MIN;
    let mut min_y = usize::MAX;
    let mut max_y = usize::MIN;

    for c in cave.keys() {
        min_x = c.x.min(min_x);
        max_x = c.x.max(max_x);
        min_y = c.y.min(min_y);
        max_y = c.y.max(max_y);
    }

    println!("Cave\n");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match cave.get(&Coord { x, y }) {
                None => print!("."),
                Some(material) => {
                    match material {
                        Material::Air => print!("."),
                        Material::Rock => print!("#"),
                        Material::Sand => print!("o"),
                        Material::Source => print!("+"),
                    }
                }
            }
        }
        println!();
    }
}

pub fn day14() {
    println!("starting day 14");

    let contents = fs::read_to_string("data/14_demo.txt").expect("Could not read file");

    let lines = contents.split('\n');
    let (y_abyss_threshold, mut cave) = build_cave(lines);

    cave.insert(Coord { x: 500, y: 0}, Material::Source);
    print_cave(cave);
}

fn build_cave(lines: Split<char>) -> (usize, HashMap<Coord, Material>) {
     lazy_static! {
        static ref LINE: Regex = Regex::new(r"((?P<x>\d+),(?P<y>\d+))+").unwrap();
    }

    let mut cave: HashMap<Coord, Material> = HashMap::new();
    let mut y_abyss_threshold = 0;

    for line in lines {
        let new_rocks = LINE.captures_iter(line)
            .inspect(|it| println!("{:?}", it))
            .map(|it| match (it.name("x"), it.name("y")) {
                (Some(x), Some(y)) => Coord {
                    x: x.as_str().parse::<usize>().unwrap(),
                    y: y.as_str().parse::<usize>().unwrap(),
                },
                _ => panic!("Malformed regex capture {:?}", it),
            })
            .inspect(|c| y_abyss_threshold = y_abyss_threshold.max(c.y))
            .collect::<Vec<Coord>>()
            .windows(2)
            .flat_map(|pair| {
                let mut res = vec![];
                let i = &pair[0];
                let j = &pair[1];
                if i.x == j.x {
                    for y in range_inclusive_updown(i.y, j.y) {
                        res.push(Coord { x: i.x, y })
                    }
                } else if i.y == j.y {
                    for x in range_inclusive_updown(i.x, j.x) {
                        res.push(Coord { x, y: i.y });
                    }
                } else {
                    panic!("Cannot create diagonal rock formation");
                }
                return res;
            })
            .collect::<Vec<_>>();
        for rock in new_rocks {
            let _ = cave.insert(rock, Material::Rock);
        }
    }
    return (y_abyss_threshold, cave)
}
