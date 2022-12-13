use std::{fs, path::Path, collections::HashSet};

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let grid = data.split("\n").map(|r| r.split("").filter(|t| t.clone() != "").map(|t| t.parse::<i8>().unwrap()).collect::<Vec<i8>>()).collect::<Vec<_>>();

    let mut visible = HashSet::<(usize, usize)>::new();

    // Left-right
    for i in 0..grid.len() {
        let mut tallest = -1;
        for j in 0..grid[i].len() {
            if tallest < grid[i][j] {
                visible.insert((i, j));
                tallest = grid[i][j];
            }
        }
    }

    // Right-left
    for i in 0..grid.len() {
        let i = grid.len() - 1 - i;
        let mut tallest = -1;
        for j in 0..grid[i].len() {
            let j = grid[i].len() - 1 - j;
            if tallest < grid[i][j] {
                visible.insert((i, j));
                tallest = grid[i][j];
            }
        }
    }

    // Top-down
    for i in 0..grid[0].len() {
        let mut tallest = -1;
        for j in 0..grid.len() {
            if tallest < grid[j][i] {
                visible.insert((j, i));
                tallest = grid[j][i];
            }
        }
    }

    // Right-left
    for i in 0..grid[0].len() {
        let i = grid[0].len() - 1 - i;
        let mut tallest = -1;
        for j in 0..grid.len() {
            let j = grid.len() - 1 - j;
            if tallest < grid[j][i] {
                visible.insert((j, i));
                tallest = grid[j][i];
            }
        }
    }

    let mut max_score = 0;

    for i in 1..(grid.len() - 1) {
        for j in 1..(grid[0].len() - 1) {
            let mut score = 1;
            let reference = grid[i][j];

            let mut counter = 1;
            for i in (1..i).rev() {
                if grid[i][j] >= reference {break;}
                counter+=1;
            }
            score = score * counter;

            counter = 1;
            for i in (i+1)..(grid.len()-1) {
                if grid[i][j] >= reference {break;}
                counter+=1;
            }
            score = score * counter;

            counter = 1;
            for j in (1..j).rev() {
                if grid[i][j] >= reference {break;}
                counter+=1;
            }
            score = score * counter;

            counter = 1;
            for j in (j+1)..(grid[0].len()-1) {
                if grid[i][j] >= reference {break;}
                counter+=1;
            }
            score = score * counter;

            if score > max_score {max_score = score;}
        }
    }

    let part1 = visible.len();
    let part2 = max_score;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
