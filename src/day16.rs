use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

#[derive(Debug, Hash, Clone)]
enum Action {
    MoveTo(String),
    OpenValve(String),
    DoNothing,
}

#[derive(Debug, Clone)]
struct BacktrackingResult {
    max_pressure_released: u32,
    best_actions: HashMap<u32, Action>,
}

#[derive(Debug, Clone)]
struct State<'a> {
    current: String,
    open_valves: Vec<String>,
    edges: &'a HashMap<String, Vec<String>>,
    flowrates: &'a HashMap<String, u32>,
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
            let empty = Vec::<String>::new();
            let currently_reachable = state.edges.get(&state.current).unwrap_or(&empty);
            let mut max_pressure_released = 0;
            let mut best_move_result: Option<BacktrackingResult> = None;
            let mut best_move_target: Option<String> = None;

            for candidate in currently_reachable.iter() {
                if *state.flowrates.get(candidate).unwrap() == 0 {
                    continue
                }
                if state.open_valves.contains(&candidate) {
                    continue
                }
                let cand_result = most_promising_candidates(
                    time_left - 1,
                    State {
                        current: candidate.clone(),
                        open_valves: state.open_valves.clone(),
                        edges: state.edges,
                        flowrates: state.flowrates,
                    },
                );
                if cand_result.max_pressure_released > max_pressure_released {
                    max_pressure_released = cand_result.max_pressure_released;
                    best_move_result = Some(cand_result);
                    best_move_target = Some(candidate.clone());
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
                let mut new_open_valves = Vec::from(state.open_valves.clone());
                new_open_valves.push(state.current.clone());
                let cand_state = State {
                    current: state.current.clone(),
                    open_valves: new_open_valves,
                    edges: state.edges,
                    flowrates: state.flowrates,
                };
                let open_result = most_promising_candidates(time_left - 1, cand_state);
                result_to_action.push((open_result, Action::OpenValve(state.current.clone())));
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
            Some((result, action)) => {

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

    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    let mut flowrates: HashMap<String, u32> = HashMap::new();
    for line in lines {
        LINE.captures(line).and_then::<Captures, _>(|cap| {
            match (cap.name("valve"), cap.name("flow"), cap.name("reachable")) {
                (Some(valve), Some(flow), Some(reachable)) => {
                    let reachables = reachable
                        .as_str()
                        .split(", ")
                        .map(String::from)
                        .collect::<Vec<String>>();
                    let flow_parsed = flow.as_str().parse().unwrap();

                    flowrates.insert(String::from(valve.as_str()), flow_parsed);
                    edges.insert(String::from(valve.as_str()), reachables);
                }
                _ => panic!("Cannot parse line {:?}", line),
            }
            None
        });
    }

    let start_state = State {
        current: String::from("AA"),
        open_valves: vec![],
        edges: &edges,
        flowrates: &flowrates,
    };
    let time_left = 20;
    let result = most_promising_candidates(time_left, start_state);
    println!(
        "Part 1: Total released pressure: {:?}",
        result.max_pressure_released
    );
    for (min, action) in result.best_actions.iter().sorted_by_key(|&it| -(*it.0 as i32)) {
        println!("{:?}: {:?}", time_left - min, action);
    }
}
