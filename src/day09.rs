use std::cmp::{max, min};
use std::fs;
use std::str::{FromStr, Split};
use itertools::Itertools;

fn dist(a: (i32, i32), b: (i32, i32)) -> f64 {
    let tmp = ((b.0 - a.0) as f64).powi(2) + ((b.1 - a.1) as f64).powi(2);
    return tmp.sqrt();
}

fn print_debug_grid(s : (i32, i32), h: (i32, i32), t: (i32, i32)) {
    let min_x = min(s.0, h.0).min(t.0).min(0);
    let max_x = max(s.0, h.0).max(t.0).max(6);
    let min_y = min(s.1, h.1).min(t.1).min(0);
    let max_y = max(s.1, h.1).max(t.1).max(5);

    // y axis is inverted for printing
    for mut y in min_y..=max_y {
        y = max_y - y;
        for x in min_x..max_x {
            if (x, y) == h {
                print!("H")
            } else if (x, y) == t {
                print!("T")
            } else if (x, y) == s {
                print!("s");
            } else {
                print!(".")
            }
        }
        println!()
    }
    println!()
}

fn print_debug_grid2(s : (i32, i32), rope: &Vec<(i32, i32)>) {
    let min_x = rope.iter().map(|(x, _)| *x).min().unwrap().min(-5);
    let max_x = rope.iter().map(|(x, _)| *x).max().unwrap().max(5);
    let min_y = rope.iter().map(|(_, y)| *y).min().unwrap().min(5);
    let max_y = rope.iter().map(|(_, y)| *y).max().unwrap().max(5);

    // y axis is inverted for printing
    for mut y in min_y..=max_y {
        y = max_y - y;
        for x in min_x..max_x {
            let mut was_rope_part = false;
            for (i, val) in rope.iter().enumerate() {
                if (x, y) == *val {
                    if i == 0 {
                        print!("H");
                    } else {
                        print!("{i}");
                    }
                    was_rope_part = true;
                    break;
                }
            }
            if !was_rope_part {
                print!(".");
            }
        }
        println!()
    }
    println!()
}


pub fn day09() {
    println!("starting day 09");

    let contents =
        fs::read_to_string("data/09_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    part_1(lines.clone());
    part_2(lines.clone());
}

fn part_1(lines: Split<char>) {
    let mut h = (0, 0);
    let s = (0, 0);
    let mut t = (0, 0);

    let mut visited: Vec<(i32, i32)> = vec![s];

    println!("== Initial State ==");
    print_debug_grid(s, h, t);

    for line in lines {
        let mut it = line.chars();
        let mov = it.next().unwrap();
        it.next();
        let times = u32::from_str(it.as_str()).unwrap();

        //println!("== {mov} {times} ==");

        for _ in 0..times {
            match mov {
                'U' => h.1 = h.1 + 1,
                'D' => h.1 = h.1 - 1,
                'L' => h.0 = h.0 - 1,
                'R' => h.0 = h.0 + 1,
                other => panic!("Unknown movement {other}")
            }

            if dist(h, t) >= 2.0 {
                let x_comp = (h.0 - t.0).clamp(-1, 1);
                let y_comp = (h.1 - t.1).clamp(-1, 1);
                let t0_new = t.0 + x_comp;
                let t1_new = t.1 + y_comp;
                t = (t0_new, t1_new);
                visited.push(t);
            }
        }
    }

    let visited_count = visited.iter().unique().count();
    println!("Fields visited by tail {visited_count}")
}

fn part_2(lines: Split<char>) {
    let s = (0, 0);

    let mut rope = vec![(0,0); 10];
    let mut visited: Vec<(i32, i32)> = vec![s];

    println!("== Initial State ==");
    print_debug_grid2(s, &rope);

    for line in lines {
        let mut it = line.chars();
        let mov = it.next().unwrap();
        it.next();
        let times = u32::from_str(it.as_str()).unwrap();

        println!("== {mov} {times} ==");

        for _ in 0..times {
            let mut h = rope[0];
            match mov {
                'U' => h.1 = h.1 + 1,
                'D' => h.1 = h.1 - 1,
                'L' => h.0 = h.0 - 1,
                'R' => h.0 = h.0 + 1,
                other => panic!("Unknown movement {other}")
            }
            rope[0] = h;

            for i in 1..rope.len() {
                let prev = rope[i-1];
                let curr = rope[i];
                if dist(prev, curr) >= 2.0 {
                    let x_comp = (prev.0 - curr.0).clamp(-1, 1);
                    let y_comp = (prev.1 - curr.1).clamp(-1, 1);
                    let curr0_new = curr.0 + x_comp;
                    let curr1_new = curr.1 + y_comp;
                    let curr_new = (curr0_new, curr1_new);
                    rope[i] = curr_new;
                }
            }
            visited.push(rope[9]);
        }
        print_debug_grid2(s, &rope);
    }

    let visited_count = visited.iter().unique().count();
    println!("Fields visited by tail {visited_count}")
}

