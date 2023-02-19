use std::cmp::{max, min};
use std::fs;
use std::str::FromStr;
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

pub fn day09() {
    println!("starting day 09");

    let contents =
        fs::read_to_string("data/09_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    let mut h = (0, 0);
    let s = (0, 0);
    let mut t = (0, 0);

    let mut visited : Vec<(i32, i32)> = vec![s];

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
                let y_comp = (h.1 - t.1 ).clamp(-1, 1);
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

