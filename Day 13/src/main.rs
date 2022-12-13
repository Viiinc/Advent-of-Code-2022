use std::{fs, path::Path, cmp::Ordering};
extern crate json;

fn compare(left: &json::JsonValue, right: &json::JsonValue) -> Ordering {
    if left.is_number() && right.is_number() {
        return left.as_number().unwrap().as_fixed_point_i64(0).unwrap().cmp(&right.as_number().unwrap().as_fixed_point_i64(0).unwrap());
    } else if left.is_array() && right.is_array() {
        let mut i = 0;
        loop {
            let result = compare(&left[i], &right[i]);
            if result == Ordering::Equal {
                i = i+1;
            } else {
                return result;
            }
            if i > left.len() && i > right.len() {return Ordering::Equal}
        }
    } else if left.is_null() && !right.is_null() {
        return Ordering::Less;
    } else if !left.is_null() && right.is_null() {
        return Ordering::Greater;
    } else if left.is_null() && right.is_null() {
        return Ordering::Equal;
    } else if left.is_number() && right.is_array() {
        return compare(&json::array![left.as_number().unwrap().as_fixed_point_i64(0).unwrap()], right);
    } else if left.is_array() && right.is_number() {
        return compare(left, &json::array![right.as_number().unwrap().as_fixed_point_i64(0).unwrap()]);
    }
    panic!("Invalid input!");
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let pairs = data.split("\n\n").map(|l| l.split_once("\n").unwrap()).collect::<Vec<_>>();

    let mut part1 = 0;

    for i in 0..pairs.len() {
        let left = json::parse(pairs[i].0).unwrap();
        let right = json::parse(pairs[i].1).unwrap();
        if compare(&left, &right) == Ordering::Less {part1 = part1 + 1 + i};
    }


    let delimiter_2 = json::parse("[[2]]").unwrap();
    let delimiter_6 = json::parse("[[6]]").unwrap();
    let mut packets = data.split("\n").filter(|s| s.len() > 0).map(|s| json::parse(s).unwrap()).collect::<Vec<_>>();
    packets.push(delimiter_2.clone());
    packets.push(delimiter_6.clone());
    packets.sort_by(compare);
    let _debug = packets.iter().map(|j| j.to_string()).collect::<Vec<_>>();
    
    let mut part2 = 1;
    for i in 0..packets.len() {
        if packets[i] == delimiter_2 || packets[i] == delimiter_6 {
            part2 *= i + 1;
        }
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
