use std::{fs, path::Path};

fn score_1(round: Vec<String>) -> i32 {
    match round[1].as_str() {
        "X" => {
            match round[0].as_str() {
                "A" => 1 + 3,
                "B" => 1,
                "C" => 1 + 6,
                _ => 0
            }
        },
        "Y" => {
            match round[0].as_str() {
                "A" => 2 + 6,
                "B" => 2 + 3,
                "C" => 2,
                _ => 0
            }
        },
        "Z" => {
            match round[0].as_str() {
                "A" => 3,
                "B" => 3 + 6,
                "C" => 3 + 3,
                _ => 0
            }
        },
        _ => 0
    }
}

fn score_2(round: Vec<String>) -> i32 {
    match round[1].as_str() {
        "X" => {
            match round[0].as_str() {
                "A" => 3,
                "B" => 1,
                "C" => 2,
                _ => 0
            }
        },
        "Y" => {
            match round[0].as_str() {
                "A" => 3 + 1,
                "B" => 3 + 2,
                "C" => 3 + 3,
                _ => 0
            }
        },
        "Z" => {
            match round[0].as_str() {
                "A" => 6 + 2,
                "B" => 6 + 3,
                "C" => 6 + 1,
                _ => 0
            }
        },
        _ => 0
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let rounds = data
    // Split into rounds
        .split("\n")
    // Get both players' choice
        .map(|e| e.split(" ").map(|r| r.to_string()).collect::<Vec<String>>());

    let part1: i32 = rounds.clone().map(|r| score_1(r)).sum();
    let part2: i32 = rounds.map(|r| score_2(r)).sum();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
