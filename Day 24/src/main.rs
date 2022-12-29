use std::{fs, path::Path, collections::HashSet};
extern crate queues;
use queues::*;

// 'O' = all directions; 'U' = All directions but up, 'n' = all directions but down, 'C' = all directions but right, 'D' = all directions but left

fn storm_at_time(grid: &Vec<Vec<char>>, i: usize, j: usize, time: usize) -> bool {
    if j == 0 || j == grid[i].len() - 1 {return true;}
    if (i==0 && j != 1) || (i == grid.len() - 1 && j != grid[i].len() - 2) {return true;}
    let wrapped_x = time % (grid[i].len() - 2);
    if wrapped_x + j > grid[i].len() - 2 {
        if grid[i][(j + wrapped_x) % (grid[i].len() - 2)] == '<' {return true;}
    } else {
        if grid[i][j + wrapped_x] == '<' {return true;}
    }
    if wrapped_x >= j {
        let x = (j as i16 - wrapped_x as i16).abs() as usize;
        if grid[i][grid[i].len() - x - 2] == '>' {return true;}
    }
    else {
        if grid[i][j - wrapped_x] == '>' {return true;}
    }
    let wrapped_y = time % (grid.len() - 2);
    if wrapped_y + i > grid.len() - 2 {
        if grid[(i + wrapped_y) % (grid.len() - 2)][j] == '^' {return true;}
    } else {
        if grid[i + wrapped_y][j] == '^' {return true;}
    }
    if wrapped_y >= i {
        let y = (i as i16 - wrapped_y as i16).abs() as usize;
        if grid[grid.len() - y - 2][j] == 'v' {return true;}
    }
    else {
        if grid[i - wrapped_y][j] == 'v' {return true;}
    }
    false
}

const CANDIDATES: [(i8, i8); 5] = [(0,1), (1,0), (-1,0), (0,-1), (0,0)];

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
    
    let grid = data.split("\n").map(|s| s.split("").filter(|q| q.len() > 0).map(|p| p.chars().nth(0).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut bfs = queue![((0, 1), 0)];

    let part1;

    let mut visited = HashSet::new();

    loop {
        let (pos, time) = bfs.remove().expect("Failure");
        if pos == (grid.len() - 1, grid[0].len() - 2) {
            part1 = time;
            break;
        }
        CANDIDATES.iter().filter(|(x,_)| pos.0 > 0 || x >= &0)
            .filter(|(x, y)| !storm_at_time(&grid, (pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize, time + 1))
            .for_each(|(x, y)| {
                let candidate = (((pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize), time + 1);
                if visited.insert(candidate) {
                    bfs.add(candidate).expect("Couldn't add to queue");
                }
            });
    }
    let mut bfs_2 = queue![((grid.len() - 1, grid[0].len() - 2), part1)];
    let mut part2;
    visited.clear();
    loop {
        let (pos, time) = bfs_2.remove().expect("Failure");
        if pos == (0, 1) {
            part2 = time;
            break;
        }
        CANDIDATES.iter().filter(|(x,_)| ((pos.0 as i8 + *x) as usize) < grid.len())
            .filter(|(x, y)| !storm_at_time(&grid, (pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize, time + 1))
            .for_each(|(x, y)| {
                let candidate = (((pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize), time + 1);
                if visited.insert(candidate) {
                    bfs_2.add(candidate).expect("Couldn't add to queue");
                }
            });
    }
    let mut bfs_3 = queue![((0, 1), part2)];
    visited.clear();
    loop {
        let (pos, time) = bfs_3.remove().expect("Failure");
        if pos == (grid.len() - 1, grid[0].len() - 2) {
            part2 = time;
            break;
        }
        CANDIDATES.iter().filter(|(x,_)| pos.0 > 0 || x >= &0)
            .filter(|(x, y)| !storm_at_time(&grid, (pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize, time + 1))
            .for_each(|(x, y)| {
                let candidate = (((pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize), time + 1);
                if visited.insert(candidate) {
                    bfs_3.add(candidate).expect("Couldn't add to queue");
                }
            });
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}