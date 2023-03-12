use crate::day15::Object::{Beacon, Sensor};
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};
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

    let sensor_coords = objects
        .iter()
        .filter(|(_, o)| match o {
            Sensor(_, _) => true,
            Beacon(_) => false,
            Object::Covered => false,
        })
        .collect::<Vec<(&Coord, &Object)>>();

    println!("Finding the only location not covered by beacons ... (this may take a long time)");

    let mut outline_coords = HashSet::new();
    for (coord, obj) in objects.iter() {
        match obj {
            Sensor(_, reach) => {
                let circle = circle_outline_plusone(coord, *reach);
                circle.iter().for_each(|it| {
                    outline_coords.insert(*it);
                });
            }
            Beacon(_) => {}
            Object::Covered => {}
        }
    }

    'search: for coord in outline_coords {
        if coord.x <= 0 || coord.y <= 0 || coord.y > target_line * 2 || coord.x > target_line * 2 {
            continue;
        }
        let mut outside_all_sensors = true;
        'sensors: for (&sensor_coord, &sensor) in sensor_coords.iter() {
            match sensor {
                Sensor(_, reach) => {
                    let dist = manhattan_dist(&coord, &sensor_coord);
                    if dist <= reach {
                        outside_all_sensors = false;
                        break 'sensors;
                    }
                }
                Beacon(_) => panic!("Invalid object type"),
                Object::Covered => panic!("Invalid object type"),
            }
        }
        if outside_all_sensors {
            println!("Found target coordinates at {:?}", coord);
            println!("Tuning frequency is {:?}", coord.x as u64 * 4000000 + coord.y as u64);
            break 'search;
        }
    }
}

// gives points on outline (radius + 1) of a circle
// uses manhattan distance
fn circle_outline_plusone(center: &Coord, radius: u32) -> Vec<Coord> {
    let mut outline = vec![];
    let real_rad = (radius + 1) as i32;

    for y in (center.y - real_rad)..=(center.y + real_rad) {
        let x_span = real_rad - (center.y - y).abs();
        if x_span == 0 {
            outline.push(Coord { x: center.x, y })
        } else {
            outline.push(Coord {
                x: center.x + x_span,
                y,
            });
            outline.push(Coord {
                x: center.x - x_span,
                y,
            });
        }
    }

    return outline;
}

#[allow(dead_code)]
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
