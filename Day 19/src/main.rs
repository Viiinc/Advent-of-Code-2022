use std::{fs, path::Path, u8::MAX};

use regex::Regex;

const TIME_LIMIT: u8 = 24;
const TIME_LIMIT_PART_2: u8 = 32;

struct Blueprint {
    id: u8,
    ore_cost: u8,
    clay_cost: u8,
    obsidian_cost: (u8, u8),
    geode_cost: (u8, u8)
}

impl Blueprint {
    fn max_count(&self, b: &BotType) -> u8 {
        match b {
            BotType::ORE => self.ore_cost.max(self.clay_cost).max(self.obsidian_cost.0).max(self.geode_cost.0),
            BotType::CLAY => self.obsidian_cost.1,
            BotType::OBSIDIAN => self.geode_cost.1,
            BotType::GEODE => MAX,
        }
    }
}

#[derive(Clone, Copy)]
enum BotType {
    ORE, CLAY, OBSIDIAN, GEODE, 
}

#[derive(Clone, Default)]
struct State {
    time: u8,
    time_limit: u8,
    ore_count: u8,
    ore_bots: u8,
    clay_count: u8,
    clay_bots: u8,
    obsidian_count: u8,
    obsidian_bots: u8,
    geode_count: u8,
    geode_bots: u8,
}

impl State {
    fn new(limit: u8) -> Self {
        Self { ore_bots: 1, time_limit: limit, ..Default::default() }
    }

    fn increment(&mut self) {
        self.time += 1;
        self.ore_count += self.ore_bots;
        self.clay_count += self.clay_bots;
        self.obsidian_count += self.obsidian_bots;
        self.geode_count += self.geode_bots;
    }

    fn create_bot(&mut self, bot_type: &BotType, blueprint: &Blueprint) {
        match bot_type {
            BotType::ORE => {
                self.ore_bots += 1;
                self.ore_count -= blueprint.ore_cost;
            }
            BotType::CLAY => {
                self.clay_bots += 1;
                self.ore_count -= blueprint.clay_cost;
            }
            BotType::OBSIDIAN => {
                self.obsidian_bots += 1;
                self.ore_count -= blueprint.obsidian_cost.0;
                self.clay_count -= blueprint.obsidian_cost.1;
            }
            BotType::GEODE => {
                self.geode_bots += 1;
                self.ore_count -= blueprint.geode_cost.0;
                self.obsidian_count -= blueprint.geode_cost.1;
            },
        }
    }

    fn can_build(&self, bot_type: &BotType, blueprint: &Blueprint) -> bool {
        match bot_type {
            BotType::ORE => self.ore_count >= blueprint.ore_cost,
            BotType::CLAY => self.ore_count >= blueprint.clay_cost,
            BotType::OBSIDIAN => self.ore_count >= blueprint.obsidian_cost.0 && self.clay_count >= blueprint.obsidian_cost.1,
            BotType::GEODE => self.ore_count >= blueprint.geode_cost.0 && self.obsidian_count >= blueprint.geode_cost.1,
        }
    }

    fn next_bot(&self, blueprint: &Blueprint) -> Vec<BotType> {
        let mut res = vec![];
        let remaining_time = self.time_limit - self.time - 1;
        if self.obsidian_count + self.obsidian_bots * remaining_time >= blueprint.geode_cost.1 && self.ore_count + self.ore_bots * remaining_time >= blueprint.geode_cost.0 {res.push(BotType::GEODE)}
        // if self.clay_count + self.clay_bots * (remaining_time) >= blueprint.obsidian_cost.1 && self.ore_count + self.ore_bots * remaining_time >= blueprint.obsidian_cost.0 && self.obsidian_bots < blueprint.max_count(&BotType::OBSIDIAN) {res.push(BotType::OBSIDIAN)}
        // if self.ore_count + self.ore_bots * (remaining_time) >= blueprint.ore_cost && self.ore_bots < blueprint.max_count(&BotType::ORE)  {res.push(BotType::ORE)}
        // if self.ore_count + self.ore_bots * (remaining_time) >= blueprint.clay_cost && self.clay_bots < blueprint.max_count(&BotType::CLAY) {res.push(BotType::CLAY)}
        if remaining_time >= 3 && self.clay_count + self.clay_bots * (remaining_time - 3) >= blueprint.obsidian_cost.1 && self.ore_count + self.ore_bots * remaining_time >= blueprint.obsidian_cost.0 && self.obsidian_bots < blueprint.max_count(&BotType::OBSIDIAN) {res.push(BotType::OBSIDIAN)}
        if remaining_time >= blueprint.ore_cost && self.ore_count + self.ore_bots * (remaining_time - blueprint.ore_cost) >= blueprint.ore_cost && self.ore_bots < blueprint.max_count(&BotType::ORE)  {res.push(BotType::ORE)}
        if remaining_time >= 5 && self.ore_count + self.ore_bots * (remaining_time - 5) >= blueprint.clay_cost && self.clay_bots < blueprint.max_count(&BotType::CLAY) {res.push(BotType::CLAY)}
        res
    }

    fn produce_bot(&mut self, blueprint: &Blueprint, bot_type: &BotType) {
        loop {
            if self.time == self.time_limit {return;}
            if self.can_build(bot_type, blueprint) {
                self.increment();
                self.create_bot(bot_type, blueprint);
                return;
            } else {
                self.increment();
            }
        }
    }
}

fn part_1_recursion(state: &mut State, blueprint: &Blueprint) -> u8 {
    if state.time == state.time_limit {
        return state.geode_count;
    }
    match state.next_bot(blueprint).iter().map(|b| {
        let mut state = state.clone();
        state.produce_bot(blueprint, b);
        part_1_recursion(&mut state, blueprint)
    }).max() {
        Some(m) => m,
        None => {
            state.increment();
            part_1_recursion(state, blueprint)
        }
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
      
    let re = Regex::new(r"Blueprint (-?\d+): Each ore robot costs (-?\d+) ore. Each clay robot costs (-?\d+) ore. Each obsidian robot costs (-?\d+) ore and (-?\d+) clay. Each geode robot costs (-?\d+) ore and (-?\d+) obsidian.").unwrap();

    let mut blueprints = vec![];

    for cap in re.captures_iter(&data) {
        blueprints.push(Blueprint{id: cap[1].parse().unwrap(), ore_cost: cap[2].parse().unwrap(), clay_cost: cap[3].parse().unwrap(), obsidian_cost: (cap[4].parse().unwrap(), cap[5].parse().unwrap()), geode_cost: (cap[6].parse().unwrap(), cap[7].parse().unwrap())});
    }

    // let temp = blueprints.iter().map(|b| b.id * part_1_recursion(&mut State::new(TIME_LIMIT), b)).collect::<Vec<_>>();

    // 988
    let part1: u16 = blueprints.iter().map(|b| b.id * part_1_recursion(&mut State::new(TIME_LIMIT), b)).map(|l| l as u16).sum();
    
    let part2 = blueprints.iter().filter(|b| b.id <= 3).map(|b| part_1_recursion(&mut State::new(TIME_LIMIT_PART_2), b) as u16).reduce(|a, b| a * b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}