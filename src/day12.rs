use std::{fs};
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn get_normalized_value(c: char) -> i32 {
    match c {
        'E' => get_normalized_value('z'),
        'S' => get_normalized_value('a'),
        other => other as i32,
    }
}

fn coord_to_direction(pred: &(usize, usize), succ: &(usize, usize)) -> char {
    match (pred.0 as i32 - succ.0 as i32, pred.1 as i32 - succ.1 as i32) {
        (0, 1) => '<',
        (0, -1) => '>',
        (1, 0) => '^',
        (-1, 0) => 'v',
        _ => '?',
    }
}

fn chardiff(a: &(usize, usize), b: &(usize, usize), grid: &Vec<Vec<char>>) -> i32 {
    let a_val = get_normalized_value(grid[a.0][a.1]);
    let b_val = get_normalized_value(grid[b.0][b.1]);

    if (a.0 as i32 - b.0 as i32).abs() > 1 || (a.1 as i32 - b.1 as i32).abs() > 1 {
        panic!("Not neighbors: {:?}, {:?}", a, b);
    }

    return (a_val - b_val).abs();
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!()
    }
}

fn reconstruct_shortest_path(start: &(usize, usize), target: &(usize, usize), cheapest_predecessor: &HashMap<(usize, usize), (usize, usize)>, grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut path = vec![vec!['.'; grid[0].len()]; grid.len()];
    let table = cheapest_predecessor.clone();
    let mut curr = target;

    path[target.0][target.1] = 'E';

    while curr != start {
        let pred = table.get(&curr).unwrap();
        path[pred.0][pred.1] = coord_to_direction(pred, curr);
        curr = pred;
    }

    return path;
}

fn neighbors(&(y, x): &(usize, usize), grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let max_y = grid.len();
    let max_x = grid[0].len();

    let mut neighbors = vec![];
    if y < max_y - 1 {
        neighbors.push((y + 1, x));
    }
    if y > 0 {
        neighbors.push((y - 1, x));
    }
    if x < max_x - 1 {
        neighbors.push((y, x + 1))
    }
    if x > 0 {
        neighbors.push((y, x - 1));
    }

    return neighbors;
}

pub fn day12() {
    println!("starting day 12");

    let contents = fs::read_to_string("data/12_demo.txt").expect("Could not read file");
    let lines = contents.split('\n');

    let mut grid: Vec<Vec<char>> = vec![];

    for line in lines {
        grid.push(line.chars().into_iter().collect::<Vec<char>>())
    }

    let mut start: (usize, usize) = (0, 0);
    let mut target: (usize, usize) = (0, 0);

    for (y, line) in grid.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            print!("{}", char);
            if char == 'S' {
                start = (y, x);
            } else if char == 'E' {
                target = (y, x);
            }
        }
        println!()
    }


    // to simplify addressing nodes, we use a tuple of vec indices as coordinates
    // (y_axis, x_axis) in
    let mut discovered: HashSet<(usize, usize)> = HashSet::new();
    discovered.insert(start);
    let mut cheapest_predecessor: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cost: HashMap<(usize, usize), u32> = HashMap::new();
    cost.insert(start, 0);

    // graphical map of movement
    let mut map = vec![vec!['.'; grid[0].len()]; grid.len()];
    map[start.0][start.1] = 'S';
    map[target.0][target.1] = 'E';

    print_map(&map);

    while !discovered.is_empty() {
        let discovered_clone = discovered.clone();
        let current = discovered_clone
            .iter()
            .map(|it| ((it), cost.get(it).unwrap_or(&u32::MAX)))
            .sorted_by_key(|x| x.1)
            .map(|x| x.0)
            .min()
            .unwrap();

        if *current == target {
            println!("Reached target");
            break;
        }

        discovered.remove(current);

        for neighbor in neighbors(current, &grid) {
            let chardiff = chardiff(current, &neighbor, &grid);
            let new_dist = cost[current] + 1;
            if new_dist < *cost.get(&neighbor).unwrap_or(&u32::MAX) && chardiff <= 1 {
                map[current.0][current.1] = coord_to_direction(current, &neighbor);
                cheapest_predecessor.insert(neighbor, *current);
                cost.insert(neighbor, new_dist);
                discovered.insert(neighbor);
            }
        }
        println!("All shortest paths:");
        print_map(&map);
    }

    println!("All shortest paths:");
    print_map(&map);
    println!("\nShortest path S->E:");
    let map_to_target = reconstruct_shortest_path(&start, &target,& cheapest_predecessor, &grid);
    print_map(&map_to_target);

    println!("Cost of reaching E: {:?}", cost.get(&target).unwrap());
}
