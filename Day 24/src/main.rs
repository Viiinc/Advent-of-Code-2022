use std::{fs, path::Path, collections::HashSet};
extern crate queues;
use queues::*;

const CANDIDATES: [(i8, i8); 5] = [(0,1), (1,0), (-1,0), (0,-1), (0,0)];

// Looks complicated, but just checks if there will be a blizzard at i,j at time 'time' (prevents us having to actually update the grid)
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

fn time_to(grid: &Vec<Vec<char>>, start_i: usize, start_j: usize, start_time: usize, end_i: usize, end_j: usize) -> usize {
    let mut bfs = queue![((start_i, start_j), start_time)];
    let mut visited = HashSet::new();
    
    loop {
        let (pos, time) = bfs.remove().expect("Failure");
        if pos == (end_i, end_j) {
            return time;
        }
        CANDIDATES.iter().filter(|(x,_)| (pos.0 > 0 || x >= &0) && (((pos.0 as i8 + *x) as usize) < grid.len()))
            .filter(|(x, y)| !storm_at_time(&grid, (pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize, time + 1))
            .for_each(|(x, y)| {
                let candidate = (((pos.0 as i8 + x) as usize, (pos.1 as i8 + y) as usize), time + 1);
                if visited.insert(candidate) {
                    bfs.add(candidate).expect("Couldn't add to queue");
                }
            });
        }
    }
    
fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
    
    let grid = data.split("\n").map(|s| s.split("").filter(|q| q.len() > 0).map(|p| p.chars().nth(0).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    let (end_i, end_j) = (grid.len() - 1, grid[0].len() - 2);

    //Part 1: 262,
    let part1 = time_to(&grid, 0, 1, 0, end_i, end_j);
    
    let back = time_to(&grid, end_i, end_j, part1, 0, 1);
    //Part 2: 785
    let part2 = time_to(&grid, 0, 1, back, end_i, end_j);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}