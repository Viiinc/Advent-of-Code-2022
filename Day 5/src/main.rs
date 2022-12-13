use std::{fs, path::Path};

use regex::Regex;

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut data_iterator = data.split("\n\n");

    let crate_input = data_iterator.next().unwrap().split("\n").map(|s| s.to_string());

    let mut crates = Vec::new();

    for i in 0..9 {
        let mut stack = crate_input.clone().filter(|r| r.chars().nth(i*4).unwrap() == '[').map(|r| r.chars().nth(i*4 + 1).unwrap()).collect::<Vec<char>>();
        stack.reverse();
        crates.push(stack);
    }

    let instructions = data_iterator.next().unwrap();

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let mut crates_1 = crates.clone();
    
    for cap in re.captures_iter(instructions) {
        let loops = cap[1].parse().unwrap();
        let source = cap[2].parse::<usize>().unwrap() - 1;
        let dest = cap[3].parse::<usize>().unwrap() - 1; 
        for _ in 0..loops {
            let sbox = crates_1[source].pop().unwrap();
            crates_1[dest].push(sbox);
        }
    };

    let part1 = crates_1.into_iter().map(|v| v.last().unwrap().clone()).collect::<String>();

    let mut crates_2 = crates.clone();
    for cap in re.captures_iter(instructions) {
        let loops = cap[1].parse().unwrap();
        let source = cap[2].parse::<usize>().unwrap() - 1;
        let dest = cap[3].parse::<usize>().unwrap() - 1;
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..loops {
            let sbox = crates_2[source].pop().unwrap();
            temp.push(sbox);
        }
        for _ in 0..loops {
            crates_2[dest].push(temp.pop().unwrap());
        }
    };

    let part2 = crates_2.into_iter().map(|v| v.last().unwrap().clone()).collect::<String>();

    println!("Part1: {}\nPart2: {}", part1, part2);
}
