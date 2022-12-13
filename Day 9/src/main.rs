use std::{fs, path::Path, collections::HashSet};

fn touching(head: &(i32, i32), tail: &(i32, i32)) -> bool {
    (head.0 - tail.0).abs() <= 1 && (head.1 - tail.1).abs() <= 1
}

fn update_rope(rope: &mut Vec<(i32,i32)>) {
    for i in 1..rope.len() {
        if touching(&rope[i], &rope[i-1]) {return;}
        rope[i].0 += (rope[i-1].0 - rope[i].0).clamp(-1, 1);
        rope[i].1 += (rope[i-1].1 - rope[i].1).clamp(-1, 1);
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut part1 = HashSet::<(i32, i32)>::new();
    let mut part2 = HashSet::<(i32, i32)>::new();

    let mut rope = Vec::with_capacity(10);
    for _ in 0..10 {
        rope.push((0,0));
    }

    for line in data.lines() {
        let (dir, steps) = line.split_once(" ").unwrap();
        let steps: i32 = steps.parse().unwrap();
        for _ in 0..steps {
            match dir {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "R" => rope[0].0 += 1,
                "L" => rope[0].0 -= 1,
                _ => unreachable!()
            }
            update_rope(&mut rope);
            part1.insert(rope[1]);
            part2.insert(rope[9]);
        }
    }

    println!("Part 1: {},\nPart 2: {}", part1.len(), part2.len());
}
