use std::{fs, path::Path, collections::HashMap};

use regex::Regex;

fn rotate(direction: &mut (i16, i16)) {
    match direction {
        (0, 1) => {direction.0 = 1; direction.1 = 0;},
        (1, 0) => {direction.0 = 0; direction.1 = -1;},
        (0, -1) => {direction.0 = -1; direction.1 = 0;},
        (-1, 0) => {direction.0 = 0; direction.1 = 1;},
        _ => unreachable!()
    }
}

fn wrap(current: (usize, usize), direction: &(i16, i16), grid: &Vec<Vec<char>>) -> (usize, usize) {
    let reverse_step = (-direction.0, -direction.1);
    let mut next: (usize, usize) = ((current.0 as i16 + reverse_step.0) as usize, (current.1 as i16 + reverse_step.1) as usize);
    while grid[next.0][next.1] != ' ' {
        next = ((next.0 as i16 + reverse_step.0) as usize, (next.1 as i16 + reverse_step.1) as usize);
    }
    ((next.0 as i16 + direction.0) as usize, (next.1 as i16 + direction.1) as usize)
}

fn score(current: (usize, usize), direction: &(i16, i16)) -> usize {
    let dir = match direction {
        (0, 1) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (-1, 0) => 3,
        _ => unreachable!(),
    };

    1000 * (current.0) + 4 * (current.1) + dir
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
      
        
    let (gridstring, instrstring) = data.split_once("\n\n").unwrap();
    
    let mut grid = gridstring.split("\n").map(|s| s.split("").filter(|q| q.len() > 0).map(|p| p.chars().nth(0).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    grid.insert(0, vec![' '; grid[0].len()]);
    grid.push(vec![' '; grid[0].len()]);
    let req_length = grid.iter().map(|l| l.len()).max().unwrap() + 2;
    grid.iter_mut().for_each(|r| {
        r.insert(0, ' ');
        while r.len() < req_length {
            r.push(' ');
        }
    });
    
    let re = Regex::new(r"(\d+)([LR\n])").unwrap();

    let mut instructions = vec![];

    for cap in re.captures_iter(instrstring) {
        instructions.push((cap[1].parse::<usize>().unwrap(), cap[2].chars().nth(0).unwrap()));
    }

    let mut current: (usize, usize) = (0,0);
    for i in 0..grid[0].len() {
        if grid[1][i] == '.' {
            current = (1, i);
            break;
        }
    }

    let start = current.clone();

    let mut direction: (i16, i16) = (0,1);

    for instr in  instructions.clone() {
        for _ in 0..instr.0 {
            let next: (usize, usize) = ((current.0 as i16 + direction.0) as usize, (current.1 as i16 + direction.1) as usize);
            match grid[next.0][next.1] {
                '#' => break,
                '.' => current = next,
                ' ' => {
                    let next = wrap(current, &direction, &grid);
                    if grid[next.0][next.1] == '#' {break;}
                    current = next;
                    // Wrap - turn around until you find another ' '
                },
                _ => unreachable!()
            }
        }
        match instr.1 {
            'R' => {
                rotate(&mut direction);
            }
            'L' => {
                for _ in 0..3 {rotate(&mut direction)}
            }
            '\n' => break,
            _ => unreachable!()
        }
    }

    // 149250
    let part1 = score(current, &direction);

    // Hardcoded monstrosity for wrapping behavior, mapping out of bounds tile + direction to next tile + direction
    let mut cube_wrap:HashMap<((usize, usize), (i16, i16)),((usize, usize), (i16, i16))> = HashMap::new();
    for i in 0..50 {
        // Top line first side
        cube_wrap.insert(((0, start.1 + i), (-1, 0)), ((3*50 + 1 + i, 1),(0,1)));
        cube_wrap.insert(((3*50 + 1 + i, 0),(0,-1)), ((1, start.1 + i), (1, 0)));
        // Top line second side
        cube_wrap.insert(((0, start.1 + i + 50), (-1, 0)), ((4*50, 1 + i),(-1, 0)));
        cube_wrap.insert(((4*50 + 1, 1 + i),(1, 0)), ((1, start.1 + i + 50), (1, 0)));
        // Left line first side TODO: shoult be inverted (I think)
        cube_wrap.insert(((1 + i, start.1 - 1), (0,-1)), ((3*50 - i, 1),(0,1)));
        cube_wrap.insert(((3*50 - i, 0),(0,-1)), ((1 + i, start.1), (0,1)));
        // Right line second side
        cube_wrap.insert(((1 + i, start.1 + 2*50), (0, 1)), ((3*50 - i, 2*50), (0,-1)));
        cube_wrap.insert(((3*50 - i, 2*50 + 1), (0,1)), ((1 + i, start.1 + 2*50 - 1), (0, -1)));
        // Bottom line second side
        cube_wrap.insert(((50 + 1, start.1 + i + 50), (1, 0)), ((50 + i + 1, 2 * 50), (0, -1)));
        cube_wrap.insert(((50 + i + 1, 2 * 50 + 1), (0, 1)), ((50, start.1 + i + 50), (-1, 0)));
        // Left line third side
        cube_wrap.insert(((50 + 1 + i, start.1 - 1), (0, -1)), ((2 * 50 + 1, 1 + i), (1, 0)));
        cube_wrap.insert(((2 * 50, 1 + i), (-1, 0)), ((50 + 1 + i, start.1), (0, 1)));
        // right line final side
        cube_wrap.insert(((3* 50 + 1 + i, start.1), (0, 1)), ((3 * 50, start.1 + i), (-1, 0)));
        cube_wrap.insert(((3 * 50 + 1, start.1 + i), (1, 0)), ((3* 50 + 1 + i, start.1 - 1), (0, -1)));
    }

    direction = (0, 1);
    current = start;

    for instr in  instructions {
        for _ in 0..instr.0 {
            let next: (usize, usize) = ((current.0 as i16 + direction.0) as usize, (current.1 as i16 + direction.1) as usize);
            match grid[next.0][next.1] {
                '#' => break,
                '.' => current = next,
                ' ' => {
                    let (next, new_dir) = *cube_wrap.get(&(next, direction)).unwrap();
                    if grid[next.0][next.1] == '#' {break;}
                    current = next;
                    direction = new_dir;
                    // Wrap - turn around until you find another ' '
                },
                _ => unreachable!()
            }
        }
        match instr.1 {
            'R' => {
                rotate(&mut direction);
            }
            'L' => {
                for _ in 0..3 {rotate(&mut direction)}
            }
            '\n' => break,
            _ => unreachable!()
        }
    }

    let part2 = score(current, &direction);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}