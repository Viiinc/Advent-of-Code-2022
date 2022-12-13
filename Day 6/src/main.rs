use std::{fs, path::Path, collections::HashSet};

fn find_delimiter(data: &str, delimiter_size: usize) -> i32 {
    for i in (delimiter_size - 1)..data.len() {
        if data.get(i-(delimiter_size - 1)..=i).unwrap().bytes().into_iter().collect::<HashSet<_>>().len() == delimiter_size {
            return (i + 1) as i32;
        }
    }
    return -1;
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let part1 = find_delimiter(&data, 4);
    let part2 = find_delimiter(&data, 14);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
