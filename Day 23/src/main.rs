use std::{fs, path::Path, collections::{HashMap, HashSet}};

fn _print_grid(grid: &Vec<Vec<char>>) {
    println!("{}", grid.iter().map(|r| r.iter().collect::<String>()).collect::<Vec<_>>().join("\n"));
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
    
    let mut grid = data.split("\n").map(|s| s.split("").filter(|q| q.len() > 0).map(|p| p.chars().nth(0).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let proposed_moves: Vec<((i16, i16),Vec<(i16,i16)>)> = vec![((-1,0), vec![(-1, -1), (-1, 0), (-1, 1)]), ((1,0), vec![(1,0), (1,1), (1, -1)]), ((0,-1), vec![(-1, -1), (0, -1), (1, -1)]), ((0,1), vec![(-1, 1), (0, 1), (1, 1)])];
    let neighbours: Vec<(i16,i16)> = vec![(1, 1), (1, 0), (0, 1), (-1, -1), (-1, 0), (0, -1), (1, -1), (-1, 1)];

    for _ in 0..60 {
        grid.insert(0, vec!['.'; grid[0].len()]);
        grid.push(vec!['.'; grid[0].len()]);
    }
    for _ in 0..60 {
        grid.iter_mut().for_each(|r| {
            r.push('.');
            r.insert(0, '.');
        })
    }

    let mut proposals = HashMap::new();
    let mut duplicates = HashSet::new();

    for r in 0..10 {
        for i in 1..(grid.len()-1) {
            for j in 1..(grid[0].len()-1) {
                if grid[i][j] == '.' {continue;}
                if neighbours.iter().all(|(x, y)| grid[(i as i16 + x) as usize][(j as i16 + y) as usize] == '.') {continue;}
                for a in 0..4 {
                    let attempt = &proposed_moves[(a + r) % 4];
                    let candidate = ((i as i16 + attempt.0.0) as usize, (j as i16 + attempt.0.1) as usize);
                    if attempt.1.iter().all(|diff| grid[(i as i16 + diff.0) as usize][(j as i16 + diff.1) as usize] == '.') {
                        // Propose candidate move
                        match proposals.insert(candidate, (i,j)) {
                            Some(_l) => {duplicates.insert(candidate);}
                            None => {}
                        }
                        break;
                    }
                }
            }
        }
        duplicates.iter().for_each(|d| {proposals.remove_entry(d);});
        proposals.iter().for_each(|(k, (i,j))| {
            grid[*i][*j] = '.';
            grid[k.0][k.1] = '#';
        });
        duplicates.clear();
        proposals.clear();
    }

    let (mut min_i, mut max_i, mut min_j, mut max_j) = (grid.len(), 0, grid[0].len(), 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                min_i = min_i.min(i);
                max_i = max_i.max(i);
                min_j = min_j.min(j);
                max_j = max_j.max(j);
            }
        }
    }

    // 3684
    let mut part1 = 0;
    for i in min_i..=max_i {
        for j in min_j..=max_j {
            if grid[i][j] == '.' {
                part1 += 1;
            }
        }
    }

    // 862
    let mut part2 = 10;

    loop {
        for i in 1..(grid.len()-1) {
            for j in 1..(grid[0].len()-1) {
                if grid[i][j] == '.' {continue;}
                if neighbours.iter().all(|(x, y)| grid[(i as i16 + x) as usize][(j as i16 + y) as usize] == '.') {continue;}
                for a in 0..4 {
                    let attempt = &proposed_moves[(a + part2 as usize) % 4];
                    let candidate = ((i as i16 + attempt.0.0) as usize, (j as i16 + attempt.0.1) as usize);
                    if attempt.1.iter().all(|diff| grid[(i as i16 + diff.0) as usize][(j as i16 + diff.1) as usize] == '.') {
                        // Propose candidate move
                        match proposals.insert(candidate, (i,j)) {
                            Some(_l) => {duplicates.insert(candidate);}
                            None => {}
                        }
                        break;
                    }
                }
            }
        }
        part2 += 1;
        duplicates.iter().for_each(|d| {proposals.remove_entry(d);});
        if proposals.len() == 0 {
            break;
        }
        proposals.iter().for_each(|(k, (i,j))| {
            grid[*i][*j] = '.';
            grid[k.0][k.1] = '#';
        });
        duplicates.clear();
        proposals.clear();
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}