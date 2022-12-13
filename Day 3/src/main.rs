use std::{fs, collections::HashSet, path::Path};

fn score(d: &char) -> i32 {
    if d.is_ascii_lowercase() {
        (d.to_ascii_lowercase() as i32) - 96
    } else {
        (d.to_ascii_uppercase() as i32) - 64 + 26
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let elves = data
    // Split into individual backpacks
        .split("\n").map(|b| b.split_at(b.len() / 2));
    // Get compartments per backpack
    let compartments = elves.map(|(a, b)| (a.split("").map(|s| s.to_string()).filter(|q| !q.eq("")).collect::<HashSet<String>>(), b.split("").map(|s| s.to_string()).filter(|q| !q.eq("")).collect::<HashSet<String>>())).collect::<Vec<_>>();
    // Find intersection per backpack
    let intersections = compartments.iter().map(|(a,b)| a.intersection(&b).last().unwrap().clone().chars().nth(0).unwrap()).collect::<Vec<_>>();
    // Convert to score
    let scores = intersections.iter().map(score).collect::<Vec<_>>();

    let backpacks = data.split("\n").map(|b| b.split("").filter(|q| !q.clone().eq("")).collect::<HashSet<_>>()).collect::<Vec<_>>();

    let mut part2 = 0;

    for i in 0..backpacks.len()/3 {
        let index = i*3;
        let b = backpacks[index+1].intersection(&backpacks[index+2]).map(|s| s.clone()).collect::<HashSet<_>>();
        let badge = backpacks[index].intersection(&b).map(|s| s.clone()).last().unwrap().clone().chars().nth(0).unwrap();
        part2 = part2 + score(&badge);
    }


    let part1: i32 = scores.iter().sum();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
