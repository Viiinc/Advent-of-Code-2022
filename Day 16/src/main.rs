use std::{fs, path::Path, collections::HashMap, u8::MAX};

use regex::Regex;

#[derive(Clone, Default)]
struct DPState {
    time: u8,
    visited: Vec<String>,
    remaining: Vec<String>,
    location: String,
}

const ROUNDS: u8 = 30;
const START_TIME_ELEFANT: u8 = ROUNDS - 26;

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
        DPState { time: self.time + time_diff, visited: vis, remaining: rem, location: next, ..Default::default() }
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

fn find_candidates(distances: &HashMap<String, HashMap<String, u8>>, time: u8, candidates: Vec<String>, visited: &String) -> Vec<String> {
    let location = visited.split(",").last().unwrap();
    let mut res = candidates.iter().filter(|c| distances.get(location).unwrap().get(*c).unwrap() + time < ROUNDS).map(|next| {
        let mut candidates = candidates.clone();
        for i in 0..candidates.len() {
            if candidates[i] == *next {
                candidates.remove(i);
                break;
            }
        }
        let visited = visited.clone() + "," + next;
        find_candidates(distances, time + distances.get(location).unwrap().get(next).unwrap(), candidates, &visited)
    }).flatten().collect::<Vec<_>>();
    if res.len() == 0 {
        res.push(visited.to_string());
    }
    res
}

fn score(path: &String, distances: &HashMap<String,HashMap<String,u8>>, pressures: &HashMap<String,i32>, memory: &mut HashMap<String,i32>) -> i32 {
    if memory.contains_key(path) {
        return *memory.get(path).unwrap();
    } else {
        let mut res = 0;
        let mut time = START_TIME_ELEFANT;
        let mut curr = "".to_string();
        for location in path.split(",") {
            if curr.len() == 0 {
                curr = location.to_string();
                continue;
            }
            let next = location.to_string();
            time += distances.get(&curr).unwrap().get(&next).unwrap();
            res += (ROUNDS - time) as i32 * pressures.get(&next).unwrap();
            curr = next;
        }
        memory.insert(path.clone(), res);
        return res;
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
    for k in pressures.keys().map(|k| indices.get(k).unwrap()) {
        for i in pressures.keys().map(|k| indices.get(k).unwrap()) {
            for j in pressures.keys().map(|k| indices.get(k).unwrap()) {
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
    let state = DPState{location: "AA".to_string(), remaining: relevant.clone(), visited: vec![], time: 0, ..Default::default()};

    // part1 = 2029
    let part1 = part_1_dp(&mut memory, &distances, &pressures, state);

    let mut memory:HashMap<String,i32> = HashMap::new();

    let part2 = find_candidates(&distances, START_TIME_ELEFANT, relevant.clone(), &"AA".to_string()).iter().map(|p| {
        let mut candidates = relevant.clone();
        p.split(",").for_each(|p_visited| {
            for i in 0..candidates.len() {
                if candidates[i] == p_visited {
                    candidates.remove(i);
                    break;
                }
            }
        });
        find_candidates(&distances, START_TIME_ELEFANT, candidates, &"AA".to_string()).iter().map(|elefant| (p.clone(), elefant.clone())).collect::<Vec<_>>()
    }).flatten().map(|(a,b)| score(&a, &distances, &pressures, &mut memory) + score(&b, &distances, &pressures, &mut memory)).max().unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}