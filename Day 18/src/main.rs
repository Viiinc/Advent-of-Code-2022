use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

fn adjacent(a: (i32, i32, i32), b: (i32, i32, i32)) -> bool {
    (a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs() == 1
}

fn close(a: (i32, i32, i32), b: (i32, i32, i32)) -> bool {
    (a.0 - b.0).abs() <= 1 && (a.1 - b.1).abs() <= 1 && (a.2 - b.2).abs() <= 1
}

fn adjacents(d: (i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    vec![(d.0 + 1, d.1, d.2),
    (d.0 - 1, d.1, d.2),
    (d.0, d.1 + 1, d.2),
    (d.0, d.1 - 1, d.2),
    (d.0, d.1, d.2 + 1),
    (d.0, d.1, d.2 - 1)]
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
                            
    let re = Regex::new(r"(-?\d+),(-?\d+),(-?\d+)").unwrap();

    let mut droplets = vec![];

    for cap in re.captures_iter(&data) {
        droplets.push((cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap()));
    }

    // 4314
    let mut part1 = droplets.len() * 6;

    let mut max = (0,0,0);
    let mut min = (20, 20, 20);
    
    for i in 0..droplets.len() {
        for j in i+1..droplets.len() {
            if adjacent(droplets[i], droplets[j]) {
                part1 -= 2;
            }
        }
        // Abusing this loop for part 2
        max = (max.0.max(droplets[i].0), max.1.max(droplets[i].1), max.2.max(droplets[i].2));
        min = (min.0.min(droplets[i].0), min.1.min(droplets[i].1), min.2.min(droplets[i].2));
    }

    let mut candidates = HashSet::<(i32, i32, i32)>::new();

    droplets.iter().for_each(|d| {
        adjacents(*d).iter().filter(|q| !droplets.iter().any(|p| p == *q)).for_each(|a| {candidates.insert(*a);});
    });

    let droplet_set = droplets.iter().map(|d| d.clone()).collect();
    let candidates = candidates.difference(&droplet_set).collect::<Vec<_>>();

    let mut confirmed_outside = HashSet::new();

    candidates.iter().for_each(|c| {
        if c.0 <= min.0 || c.1 <= min.1 || c.2 <= min.2 || c.0 >= max.0 || c.1 >= max.1 || c.2 >= max.2 {
            confirmed_outside.insert(**c);
        }
    });

    // Spread from known outside blocks to find all outside-adjacent blocks
    loop {
        let size = confirmed_outside.len();
        let mut additional = HashSet::new();
        confirmed_outside.iter().for_each(|d| {
            let new_candidates = adjacents(*d);
            new_candidates.iter().filter(|c| !droplet_set.contains(*c) && droplet_set.iter().any(|q| close(**c, *q))).for_each(|c| {additional.insert(c.clone());});
        });
        additional.iter().for_each(|c| {confirmed_outside.insert(*c);});
        if size == confirmed_outside.len() {break;}
    }

    let part2: i32 = confirmed_outside.iter().map(|c| droplets.iter().filter(|q| adjacent(**q, *c)).count() as i32).sum();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}