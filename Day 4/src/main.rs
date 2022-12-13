use std::{fs, path::Path};

fn contained_in_range(low: i32, high: i32, test: i32) -> bool {
    test >= low && test <= high
}

fn contained(a: i32, b: i32, c: i32, d: i32) -> bool {
    (contained_in_range(a, b, c) && contained_in_range(a, b, d)) || (contained_in_range(c, d, a) && contained_in_range(c, d, b))
}

fn overlap(a: i32, b: i32, c: i32, d: i32) -> bool {
    (contained_in_range(a, b, c) || contained_in_range(a, b, d)) || (contained_in_range(c, d, a) || contained_in_range(c, d, b))
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let elves = data.split("\n").map(|s| s.split(",").map(|q| q.split("-")).flatten().map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>());

    let part1 = elves.clone().filter(|r| contained(r[0], r[1], r[2], r[3])).count();
    let part2 = elves.clone().filter(|r| overlap(r[0], r[1], r[2], r[3])).count();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
