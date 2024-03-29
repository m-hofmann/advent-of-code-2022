use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(Debug, Hash, Clone)]
enum Action<'a> {
    MoveTo(&'a str),
    OpenValve(&'a str),
    DoNothing,
}

#[derive(Debug, Clone)]
struct BacktrackingResult<'a > {
    max_pressure_released: u32,
    best_actions: HashMap<u32, Action<'a>>,
}

#[derive(Debug, Clone)]
struct State<'a> {
    current: &'a str,
    open_valves: Vec<&'a str>,
    edges: &'a HashMap<&'a str, Vec<&'a str>>,
    flowrates: &'a HashMap<&'a str, u32>,
}

// returns a tuple oflet (mut max_pressure_relea
// - sum_pressure_released (selection criterion)
// - HashMap <time_left -> Action taken in this minute>
// - Vec of currently open valves
fn most_promising_candidates(time_left: u32, state: State) -> BacktrackingResult {
    if time_left == 0 {
        return BacktrackingResult {
            max_pressure_released: 0,
            best_actions: HashMap::new(),
        };
    } else {
        // at the current node there are 3 options we can take
        // move to next node, do nothing, or open valve
        let mut result_to_action: Vec<(BacktrackingResult, Action)> = vec![];

        // simulate move
        {
            let empty = Vec::<&str>::new();
            let currently_reachable = state.edges.get(&state.current).unwrap_or(&empty);
            let mut max_pressure_released = 0;
            let mut best_move_result: Option<BacktrackingResult> = None;
            let mut best_move_target: Option<&str> = None;

            for candidate in currently_reachable.iter() {

                let cand_result = most_promising_candidates(
                    time_left - 1,
                    State {
                        current: candidate,
                        open_valves: state.open_valves.clone(),
                        edges: state.edges,
                        flowrates: state.flowrates,
                    },
                );
                if cand_result.max_pressure_released > max_pressure_released {
                    max_pressure_released = cand_result.max_pressure_released;
                    best_move_result = Some(cand_result);
                    best_move_target = Some(candidate);
                }
            }
            if let (Some(result), Some(target)) = (best_move_result, best_move_target) {
                result_to_action.push((result, Action::MoveTo(target)));
            }
        }

        // do nothing case
        {
            let cand_result = most_promising_candidates(time_left - 1, state.clone());
            result_to_action.push((cand_result, Action::DoNothing));
        }

        // open valve case
        {
            if !state.open_valves.contains(&state.current) {
                let mut new_open_valves = state.open_valves.clone();
                new_open_valves.push(state.current);
                let cand_state = State {
                    current: state.current,
                    open_valves: new_open_valves,
                    edges: state.edges,
                    flowrates: state.flowrates,
                };
                let open_result = most_promising_candidates(time_left - 1, cand_state);
                result_to_action.push((open_result, Action::OpenValve(state.current)));
            }
        }

        let current_pressure_release: u32 = state
            .open_valves
            .iter()
            .filter_map(|it| state.flowrates.get(it))
            .sum();

        let best_outcome = result_to_action.iter()
            .max_by_key(|(result, _)| result.max_pressure_released);

        match best_outcome {
            Some((result, _)) => {

                // let mut new_actions = result.best_actions.clone();
                // new_actions.insert(time_left, action.clone());
                BacktrackingResult {
                    max_pressure_released: result.max_pressure_released + current_pressure_release,
                    best_actions: HashMap::new()
                }
            },
            None => panic!("Cannot backtrack further")
        }
    }
}

pub fn day16() {
    println!("starting day 16");

    let contents = fs::read_to_string("data/16_demo.txt").expect("Could not read file");

    let lines = contents.split('\n');

    lazy_static! {
        static ref LINE: Regex = Regex::new(r"Valve (?P<valve>\w+) has flow rate=(?P<flow>\d+); tunnels lead to valves (?P<reachable>(\w+(,\s)?)+)").unwrap();
    }

    let mut edges: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut flowrates: HashMap<&str, u32> = HashMap::new();
    for line in lines {
        LINE.captures(line).and_then::<Captures, _>(|cap| {
            match (cap.name("valve"), cap.name("flow"), cap.name("reachable")) {
                (Some(valve), Some(flow), Some(reachable)) => {
                    let reachables = reachable
                        .as_str()
                        .split(", ")
                        .collect::<Vec<&str>>();
                    let flow_parsed = flow.as_str().parse().unwrap();

                    flowrates.insert(valve.as_str(), flow_parsed);
                    edges.insert(valve.as_str(), reachables);
                }
                _ => panic!("Cannot parse line {:?}", line),
            }
            None
        });
    }

    let start_state = State {
        current: "AA",
        open_valves: vec![],
        edges: &edges,
        flowrates: &flowrates,
    };

    // Floyd-Warshall to collapse graph to a size that is more suitable for backtracking
    let mut shortest_paths: HashMap<&str, HashMap<&str, usize>> = HashMap::new();
    // initialize existing edges to weight 1 (= reachable in 1 minute)
    for (&node, adjacent) in edges.iter() {
        if !shortest_paths.contains_key(node) {
            shortest_paths.insert(node, HashMap::new());
        }
        shortest_paths.entry(node).or_default().insert(node, 0);
        for &neighbor in adjacent.into_iter() {
            if !shortest_paths.contains_key(neighbor) {
                shortest_paths.insert(neighbor, HashMap::new());
            }
            // undirected graph: Add both (node -> neighbor) as well as (neighbor -> node)
            shortest_paths.entry(node).and_modify(|map| {
                map.insert(neighbor, 1);
            });
            shortest_paths.entry(neighbor).and_modify(|map| {
                map.insert(node, 1);
            });
        }
    }
    let nodes_only : Vec<&str> = shortest_paths.keys().copied().collect();
    for &k in &nodes_only {
        for &i in &nodes_only {
            if !shortest_paths[i].contains_key(k) {
                continue;
            }

            for &j in &nodes_only {
                if i == j || !shortest_paths[k].contains_key(j) {
                    continue;
                }
                let dist_i_k = shortest_paths[i][k];
                let dist_k_j = shortest_paths[k][j];
                match shortest_paths[i].contains_key(j) {
                    true => {
                        let dist_i_j = shortest_paths[i][j];
                        if dist_i_j > dist_i_k + dist_k_j {
                            shortest_paths.entry(i).and_modify(|map| {
                                map.insert(j, dist_i_k + dist_k_j);
                            });
                        }
                    }
                    false =>  {
                        shortest_paths.entry(i).or_default().insert(j, dist_i_k + dist_k_j);
                    }
                }
            }
        }
    }
    println!("shortest paths {:?}", shortest_paths);

    let time_left = 17;
    let result = most_promising_candidates(time_left, start_state);
    println!(
        "Part 1: Total released pressure: {:?}",
        result.max_pressure_released
    );
    for (min, action) in result.best_actions.iter().sorted_by_key(|&it| -(*it.0 as i32)) {
        println!("{:?}: {:?}", time_left - min, action);
    }
}
