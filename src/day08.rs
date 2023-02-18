use std::fs;
use std::str::FromStr;

fn is_visible_naive(grid: Vec<Vec<u8>>, x: usize, y: usize) -> bool {
    // max grid size (inclusive)
    let max_y = grid.len()-1;
    let max_x = grid[0].len();

    if x == 0 || x == max_x
        || y == 0 || y == max_y {
        // edge trees are always visible
        return true;
    } else {
        // TODO recurse & memoize for speedup
        let height = grid[y][x];

        let vis_from_left = !grid[y].as_slice()[0..x].iter()
            .any(|&other_height| other_height >= height);
        let vis_from_right = !grid[y].as_slice()[x+1..=max_y].iter()
            .any(|&other_height| other_height >= height);
        let vis_from_top = !grid.iter()
            .enumerate()
            .filter(|(index, _)| *index < y)
            .map(|it| it.1[x])
            .any(|other_height| other_height >= height);
        let vis_from_bottom = !grid.iter()
            .enumerate()
            .filter(|(index, _)| *index > y)
            .map(|it| it.1[x])
            .any(|other_height| other_height >= height);

        return vis_from_left || vis_from_right || vis_from_top || vis_from_bottom;
    }
}

pub fn day08() {
    println!("starting day 08");

    let contents =
        fs::read_to_string("data/08_input.txt").expect("Could not read file");

    let lines = contents.split('\n');

    let mut grid : Vec<Vec<u8>> = vec![];
    for line_raw in lines {
        let mut line_conv = vec![];
        for i in 0..line_raw.len() {
            line_conv.push(u8::from_str(&line_raw[i..i+1]).unwrap())
        }
        grid.push(line_conv);
    }

    println!("Grid: \n{:?}", grid);

    let mut visible_count = 0;
    for (i, _) in grid.iter().enumerate() {
        for (j, _) in grid[i].iter().enumerate() {
            let is_visible = is_visible_naive(grid.clone(), j, i);
            if is_visible {
                visible_count += 1;
            }
        }
    }

    println!("Number of visible trees {:?}", visible_count);

}

