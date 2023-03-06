use crate::day15::Object::{Beacon, Sensor};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Hash, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
}

impl PartialEq<Self> for Coord {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl Eq for Coord {}

#[derive(Debug, Hash, Clone, Copy)]
enum Object {
    // beacon location and sensing distance (distance to beacon)
    Sensor(Coord, u32),
    Beacon(Coord),
    Covered,
}

pub fn day15() {
    println!("starting day 15");

    let contents = fs::read_to_string("data/15_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    lazy_static! {
        static ref LINE: Regex = Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)").unwrap();
    }

    let mut objects: HashMap<Coord, Object> = HashMap::new();
    for line in lines {
        LINE.captures(line).and_then::<Captures, _>(|cap| {
            match (
                cap.name("sx"),
                cap.name("sy"),
                cap.name("bx"),
                cap.name("by"),
            ) {
                (Some(sx), Some(sy), Some(bx), Some(by)) => {
                    let sensor_coord = Coord {
                        x: sx.as_str().parse().unwrap(),
                        y: sy.as_str().parse().unwrap(),
                    };
                    let beacon_coord = Coord {
                        x: bx.as_str().parse().unwrap(),
                        y: by.as_str().parse().unwrap(),
                    };
                    objects.insert(
                        sensor_coord.clone(),
                        Sensor(beacon_coord, manhattan_dist(&sensor_coord, &beacon_coord)),
                    );
                    objects.insert(beacon_coord.clone(), Beacon(sensor_coord));
                }
                _ => panic!("Cannot parse line {:?}", line),
            }
            None
        });
    }

    println!("Parsed input - investigating area covered by sensors.");

    let target_line = 2000000;
    let mut staging = HashMap::new();
    for (coord, object) in objects.iter() {
        staging.insert(*coord, *object);
        match object {
            Sensor(beacon, _) => {
                let dist = (coord.x - beacon.x).abs() + (coord.y - beacon.y).abs();

                for y in (coord.y - dist)..=(coord.y + dist) {
                    // optimization: only populate target line to save memory and runtime
                    if y != target_line {
                        continue
                    }

                    let x_span = dist - (coord.y - y).abs();

                    for x in (coord.x - x_span)..=(coord.x + x_span) {
                        if !staging.contains_key(&Coord { x, y }) {
                            staging.insert(Coord { x, y }, Object::Covered);
                        }
                    }
                }
            }
            Beacon(_) => {}
            Object::Covered => {}
        }
    }

    objects = staging;

    println!("Calculated coverage. Finding covered area on line {target_line}");
    //print_map(&mut objects);

    let line_exclusions = objects
        .iter()
        .filter(|(coord, _)| coord.y == target_line)
        .filter(|(_, &object)| match object {
            Sensor(_, _) => false,
            Beacon(_) => false,
            Object::Covered => true,
        })
        .filter(|(coord, _)| coord.y == target_line)
        .count();
    println!(
        "Part 1: In row where y={target_line}, {line_exclusions} positions cannot contain a beacon"
    );

    let max_size = 2 * target_line;

    let sensor_coords = objects
        .iter()
        .filter(|(_, o)| match o {
            Sensor(_, _) => true,
            Beacon(_) => false,
            Object::Covered => false,
        })
        .collect::<Vec<(&Coord, &Object)>>();

    println!("Finding the only location not covered by beacons ... (this may take a long time)");

    // accidentally ... volumetric?
    let mut target: Option<Coord> = None;
    'search: for y in 0..=max_size {
        if y % 500 == 0 {
            println!("y is at {y} y");
        }
        let mut x = 0;
        'line: while x <= max_size {
            let mut outside_all_sensors = true;
            'sensors: for (coord, sensor) in sensor_coords.iter() {
                match sensor {
                    Sensor(_, reach) => {
                        let dist = manhattan_dist(&Coord { x, y }, coord);
                        if dist <= *reach {
                            // if we reached the outer border of an area covered by a sensor
                            // we can jump to the "opposite site" of the covered area immediately
                            // and do not have to investigate every point within its bounds
                            if x < coord.x {
                                let x_dist = (coord.x - x).abs();
                                x += 2 * x_dist;
                            }
                            outside_all_sensors = false;
                            break 'sensors;
                        }
                    }
                    Beacon(_) => panic!("Invalid object type"),
                    Object::Covered => panic!("Invalid object type") ,
                }
            }
            if outside_all_sensors {
                target = Some(Coord { x, y });
                println!("Found target coordinates at {:?}", target);
                break 'search;
            }
            x += 1;
        }
    }

    match target {
        Some(coord) => {
            let tuning_freq = coord.x * 4000000 + coord.y;
            println!("Part 2: Tuning frequency is {tuning_freq}");
        }
        None => panic!("Search for distress signal unsuccessful"),
    }
    println!()
}

fn print_map(objects: &mut HashMap<Coord, Object>) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for (coord, _) in objects.iter() {
        min_x = min_x.min(coord.x);
        max_x = max_x.max(coord.x);
        min_y = min_y.min(coord.y);
        max_y = max_y.max(coord.y);
    }

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match objects.get(&Coord { x, y }) {
                Some(material) => match material {
                    Sensor(_, _) => print!("S"),
                    Beacon(_) => print!("B"),
                    Object::Covered => print!("#"),
                },
                None => print!("."),
            }
        }
        println!();
    }
}

fn manhattan_dist(a: &Coord, b: &Coord) -> u32 {
    return (a.x - b.x).abs() as u32 + (a.y - b.y).abs() as u32;
}
