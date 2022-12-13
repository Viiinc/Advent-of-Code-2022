use std::{fs, path::Path};

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let elves = data
    // Split into individual elves
        .split("\n\n")
    // Get snacks for each Elf
        .map(|e| e.split("\n").map(|f| f.parse::<i32>().unwrap()))
    // Sum up the snacks for each elf
        .map(|c| c.sum::<i32>());

    let mut sorted_elves: Vec<i32> = elves.collect::<Vec<i32>>();

    sorted_elves.sort_by(|a, b| b.cmp(a));

    let part1 = sorted_elves[0];
    let part2 = sorted_elves[0] + sorted_elves[1] + sorted_elves[2];

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
