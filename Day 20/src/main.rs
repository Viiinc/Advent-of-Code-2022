use std::{fs, path::Path};

use regex::Regex;

const KEY: i64 = 811589153;
const ITERATIONS: usize = 10;

fn mix(original: &Vec<(i64, usize)>, list: &mut Vec<(i64, usize)>, times: usize) {
    for _ in 0..times {
        for i in 0..original.len() {
            for j in 0..list.len() {
                if original[i] == list[j] {
                    let mut new_position = j as i64 + list.remove(j).0;
                    if new_position < 0 {
                        // Do this properly
                        new_position = list.len() as i64 - (new_position.abs() % list.len() as i64);
                        // new_position = list.len() as i64 + new_position;
                    }
                    if new_position > list.len() as i64 {
                        new_position = new_position % list.len() as i64;
                    }
                    list.insert(new_position as usize, original[i]);
                    break;
                }
            }
        }
    }
}

fn score(list: &Vec<(i64, usize)>) -> i64 {
    let mut start = 0;
    for i in 0..list.len() {
        if list[i].0 == 0 {
            start = i;
            break;
        }
    }

    let indices = vec![1000, 2000, 3000];
    let _elements = indices.iter().map(|i| list[(start + i) % list.len()].0).collect::<Vec<_>>();

    indices.iter().map(|i| list[(start + i) % list.len()].0).sum()
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
      
    let re = Regex::new(r"(-?\d+)").unwrap();

    let mut list: Vec<(i64, usize)> = vec![];

    let mut id = 1;
    for cap in re.captures_iter(&data) {
        list.push((cap[1].parse().unwrap(), id));
        id += 1;
    }

    let original = list.clone();

    mix(&original, &mut list, 1);

    // 17490
    let part1 = score(&list);

    let mut real_list = original.iter().map(|(i, v)| (i * KEY, *v)).collect::<Vec<_>>();
    let real_original = real_list.clone();

    mix(&real_original, &mut real_list, ITERATIONS);
    
    let part2: i64 = score(&real_list);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}