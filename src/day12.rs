use std::collections::{HashMap, HashSet};
use std::fs;

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

fn is_valid_chardiff(from: &(usize, usize), to: &(usize, usize), grid: &Vec<Vec<char>>) -> bool {
    let from_val = get_normalized_value(grid[from.0][from.1]);
    let to_val = get_normalized_value(grid[to.0][to.1]);

    if (from.0 as i32 - to.0 as i32).abs() > 1 || (from.1 as i32 - to.1 as i32).abs() > 1 {
        panic!("Not neighbors: {:?}, {:?}", from, to);
    }

    let diff = to_val as i32 - from_val as i32;
    if diff == 1 {
        return true;
    } else if diff > 1 {
        return false
    } else {
        // need to read instructions carefully or spend hours debugging:
        // (This also means that the elevation of the destination square can be much lower than the elevation of your current square.)
        return true;
    }
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            print!("{}", c);
        }
        println!()
    }
}

fn reconstruct_shortest_path(
    start: &(usize, usize),
    target: &(usize, usize),
    cheapest_predecessor: &HashMap<(usize, usize), (usize, usize)>,
    grid: &Vec<Vec<char>>,
) -> Vec<Vec<char>> {
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

    let contents = fs::read_to_string("data/12_input.txt").expect("Could not read file");
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
    let mut candidates: HashSet<(usize, usize)> = HashSet::new();
    candidates.insert(start);
    let mut visited : HashSet<(usize, usize)> = HashSet::new();
    let mut cheapest_predecessor: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut cost: HashMap<(usize, usize), u32> = HashMap::new();
    cost.insert(start, 0);

    // graphical map of movement
    let mut map = vec![vec!['.'; grid[0].len()]; grid.len()];
    map[start.0][start.1] = 'S';
    map[target.0][target.1] = 'E';

    print_map(&map);

    while !candidates.is_empty() {
        let candidates_clone = candidates.clone();
        let current = candidates_clone
            .iter()
            .map(|it| ((it), cost.get(it).unwrap_or(&u32::MAX)))
            .sorted_by_key(|x| x.1)
            .map(|x| x.0)
            .next().unwrap();

        if *current == target {
            println!("Reached target");
            break;
        }

        candidates.remove(current);
        visited.insert(*current);

        for neighbor in neighbors(current, &grid) {
            let valid_field = is_valid_chardiff(current, &neighbor, &grid);
            let new_dist = cost[current] + 1;
            if new_dist < *cost.get(&neighbor).unwrap_or(&u32::MAX)
                && valid_field
                {
                map[current.0][current.1] = coord_to_direction(current, &neighbor);
                cheapest_predecessor.insert(neighbor, *current);
                cost.insert(neighbor, new_dist);
                candidates.insert(neighbor);
            }
        }
    }

    println!("All shortest paths:");
    print_map(&map);
    println!("\nShortest path S->E:");
    let map_to_target = reconstruct_shortest_path(&start, &target,& cheapest_predecessor, &grid);
    print_map(&map_to_target);

    println!("Cost of reaching E: {:?}", cost.get(&target).unwrap());
}
