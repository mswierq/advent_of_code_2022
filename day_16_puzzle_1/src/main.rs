use regex::Regex;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use std::{fs::File, io};

type ValveId = String;

struct Valve {
    id: ValveId,
    tunnels: Vec<ValveId>,
    flow: i32,
}

impl FromStr for Valve {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"Valve ([A-Z]{2}) has flow rate=(\d+); tunnel(?:s?) lead(?:s?) to valve(?:s?) (.*)",
        )
        .unwrap();
        let re_tunnels = Regex::new(r"([A-Z]{2})").unwrap();
        let cap = re.captures(s).unwrap();
        let id = cap[1].to_owned();
        let flow = cap[2].parse::<i32>().unwrap();
        let tunnels = re_tunnels
            .captures_iter(&cap[3])
            .map(|cap| cap[1].to_owned())
            .collect::<Vec<ValveId>>();
        Ok(Self { id, tunnels, flow })
    }
}

enum Action {
    Move { id: ValveId },
    Open { id: ValveId, flow: i32 },
}

fn find_max_pressure(valves: HashMap<ValveId, Valve>) -> i32 {
    let mut queue = VecDeque::new();
    let mut closed_valves: BTreeMap<_, _> = valves.iter().map(|id, valve| {
        (valve.flow, id.clone())
    }).collect();

    for id in &valves.get("AA").unwrap().tunnels {
        queue.push_back((
            Action::Move { id: id.clone() },
            0,
            0,
            0,
            is_open.clone(),
            "AA".to_owned(),
        ));
    }
    let mut max_pressure = 0;

    while !queue.is_empty() {
        let (action, time, rate, pressure, mut is_open, previous_id) = queue.pop_front().unwrap();

        let pressure = pressure + rate;
        let time = time + 1;
        let mut rate = rate;

        if time == 30 || is_open.iter().all(|(_, open)| *open) {
            let pressure = pressure + (30 - time) * rate;
            max_pressure = i32::max(max_pressure, pressure);
            continue;
        }

        let valve_id = match action {
            Action::Move { id } => {
                let flow = valves.get(&id).unwrap().flow;
                let is_valve_open = is_open.get_mut(&id).unwrap();
                if !*is_valve_open {
                    queue.push_back((
                        Action::Open {
                            id: id.clone(),
                            flow,
                        },
                        time,
                        rate.clone(),
                        pressure,
                        is_open.clone(),
                        id.clone(),
                    ));
                }
                id
            }
            Action::Open { id, flow } => {
                rate += flow;
                *is_open.get_mut(&id).unwrap() = true;
                id
            }
        };

        for next_id in &valves.get(&valve_id).unwrap().tunnels {
            if next_id != &previous_id {
                if *next_id != valve_id {
                    queue.push_back((
                        Action::Move {
                            id: next_id.clone(),
                        },
                        time,
                        rate,
                        pressure,
                        is_open.clone(),
                        valve_id.clone(),
                    ));
                }
            }
        }
    }
    max_pressure
}

fn main() -> io::Result<()> {
    let input_path = std::env::args().nth(1).expect("No input file given!");
    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let valves = reader
        .lines()
        .map(|line| {
            let valve = Valve::from_str(&line.unwrap()).unwrap();
            (valve.id.clone(), valve)
        })
        .collect::<HashMap<ValveId, Valve>>();

    println!("Max pressure is {}", find_max_pressure(valves));

    Ok(())
}
