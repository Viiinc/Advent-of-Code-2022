use std::{fs, path::Path, collections::HashMap};

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let rocks = data.split("\n").map(|r| r.split(" -> ").map(|p| p.split_once(",").unwrap()).map(|(i,j)| (i.parse::<i32>().unwrap(), j.parse::<i32>().unwrap())).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut blocks: HashMap<(i32, i32),char> = HashMap::new();

    let mut void = 0;
    for path in rocks {
        let mut curr = path[0];
        blocks.insert(curr, '#');
        for i in 1..path.len() {
            let next = path[i];
            while curr != next {
                curr.0 += (next.0 - curr.0).clamp(-1, 1);
                curr.1 += (next.1 - curr.1).clamp(-1, 1);
                blocks.insert(curr, '#');
            }
        }
        if curr.1 > void {void = curr.1;}
    }

    let mut part1 = 0;

    'outer: loop {
        let mut sand = (500,0);
        loop {
            if sand.1 > void {break 'outer;}
            if !blocks.contains_key(&(sand.0, sand.1 + 1)) {
                sand.1 = sand.1 + 1;
            } else if !blocks.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !blocks.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                blocks.insert(sand, 'o');
                break;
            }
        }
        part1 = part1 + 1;
    }
    
    let mut part2 = part1;

    loop {
        let mut sand = (500,0);
        loop {
            if sand.1 > void {
                blocks.insert(sand, 'o');
                break;
            }
            if !blocks.contains_key(&(sand.0, sand.1 + 1)) {
                sand.1 = sand.1 + 1;
            } else if !blocks.contains_key(&(sand.0 - 1, sand.1 + 1)) {
                sand = (sand.0 - 1, sand.1 + 1);
            } else if !blocks.contains_key(&(sand.0 + 1, sand.1 + 1)) {
                sand = (sand.0 + 1, sand.1 + 1);
            } else {
                blocks.insert(sand, 'o');
                break;
            }
        }
        part2 = part2 + 1;
        if blocks.contains_key(&(500,0)) {break;}
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
