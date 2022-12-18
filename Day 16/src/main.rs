use std::{fs, path::Path, collections::HashMap, u8::MAX};

use regex::Regex;

#[derive(Clone)]
struct DPState {
    time: u8,
    visited: Vec<String>,
    remaining: Vec<String>,
    location: String,
}

const ROUNDS: u8 = 30;

impl DPState {
    fn to_string(&self) -> String {
        self.visited.join(",") + "," + &self.time.to_string() + &self.location.to_string()
    }

    fn update(&self, next: String, distances: &HashMap<String, HashMap<String, u8>>) -> Self {
        let mut vis = self.visited.clone();
        vis.push(self.location.clone());
        vis.sort();
        let mut rem = self.remaining.clone();
        for i in 0..rem.len() {
            if rem[i] == next {
                    rem.remove(i);
                break;
            }
        }
        let time_diff = distances.get(&self.location).unwrap().get(&next).unwrap();
        DPState { time: self.time + time_diff, visited: vis, remaining: rem, location: next }
    }

    fn score(&self, pressures: &HashMap<String,i32>) -> i32 {
            (self.visited.iter().map(|v| pressures.get(v).unwrap()).sum::<i32>() + pressures.get(&self.location).unwrap()) * (ROUNDS - self.time) as i32
    }
}
    
fn part_1_dp(memory: &mut HashMap<String, i32>, distances: &HashMap<String, HashMap<String, u8>>, pressures: &HashMap<String, i32>, state: DPState) -> i32 {
        let key = state.to_string();
        if memory.contains_key(&key) {
                *memory.get(&key).unwrap()
            } else {
                    // let candidates = state.remaining.iter().filter(|s| distances.get(&state.location).unwrap().get(*s).unwrap() <= &)
                    match state.remaining.iter().filter(|s| distances.get(&state.location).unwrap().get(*s).unwrap() <= &(ROUNDS - state.time)).map(|next| {
                            part_1_dp(memory, distances, pressures, state.update(next.clone(), distances))
            + (state.visited.iter().map(|v| pressures.get(v).unwrap()).sum::<i32>() + pressures.get(&state.location).unwrap()) * *(distances.get(&state.location).unwrap().get(next).unwrap()) as i32
    }).max() {
        Some(res) => {
                memory.insert(key, res);
                res
        },
        None => {
                let res = state.score(pressures);
                memory.insert(key, res);
                res
            }
        }
    }
}
                        
fn main() {
        let data = fs::read_to_string(Path::new("src/input.txt"))
            .expect("Should have been able to read the file");
                            
    let re = Regex::new(r"Valve ([A-Z]{2}) has flow rate=(-?\d+); tunnels? leads? to valves? ([A-Z, ]+)").unwrap();

    let mut valves = HashMap::new();
    let mut pressures = HashMap::new();

    for cap in re.captures_iter(&data) {
            let valve = cap[1].to_string();
            let rate = cap[2].parse::<i32>().unwrap();
            let connected = cap[3].split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        valves.insert(valve.clone(), connected);
        pressures.insert(valve.clone(), rate);
    }

    let relevant = pressures.iter().filter(|(_,v)| **v > 0).map(|(k,_)| k.clone()).collect::<Vec<_>>();

    let mut distance_matrix = vec![vec![MAX/2; valves.len()]; valves.len()];
    let mut indices = HashMap::new();
    let mut index = 0;
    for vert in pressures.keys() {
        indices.insert(vert, index);
        index += 1;
    }
    for vert in valves {
        let index = indices.get(&vert.0).unwrap();
        for other in vert.1 {
            distance_matrix[*index][*indices.get(&other).unwrap()] = 1;
        }
        distance_matrix[*index][*index] = 0;
    }
    for k in pressures.keys() {
        for i in pressures.keys() {
            for j in pressures.keys() {
                let (k, i, j) = (indices.get(k).unwrap(), indices.get(i).unwrap(), indices.get(j).unwrap());
                if distance_matrix[*i][*j] > distance_matrix[*i][*k] + distance_matrix[*k][*j] {
                    distance_matrix[*i][*j] = distance_matrix[*i][*k] + distance_matrix[*k][*j];
                }
            }
        }
    }
    let mut distances = relevant.iter().map(|v| {
        (v.to_string(), relevant.iter().map(|o| (o.clone(), distance_matrix[*indices.get(v).unwrap()][*indices.get(o).unwrap()] + 1)).collect::<HashMap<_,_>>())
    }).collect::<HashMap<_,_>>();
    distances.insert("AA".to_string(), relevant.iter().map(|o| (o.clone(), distance_matrix[*indices.get(&"AA".to_string()).unwrap()][*indices.get(o).unwrap()] + 1)).collect::<HashMap<_,_>>());

    // Memory for DP
    let mut memory:HashMap<String, i32> = HashMap::new();
    let state = DPState{location: "AA".to_string(), remaining: relevant.clone(), visited: vec![], time: 0};

    // part1 = 2029
    let part1 = part_1_dp(&mut memory, &distances, &pressures, state);
    let part2 = 0;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}