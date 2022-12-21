use std::{fs, path::Path, collections::HashMap};
       
const ROCKS: usize = 2022;
const WIDTH: usize = 7;
const ITERATIONS_PART_2: u64 = 1000000000000;

fn height(rocks: u64, repeat_start: (usize, usize), repeat_end: (usize, usize), partial: &HashMap<usize, u64>) -> u64 {
    let count = (rocks - repeat_start.0 as u64) / (repeat_end.0 as u64 - repeat_start.0 as u64);
    let remaining = ((rocks - repeat_start.0 as u64) % (repeat_end.0 as u64 - repeat_start.0 as u64)) as usize;
    let res = count * ((repeat_end.1 - repeat_start.1) as u64) + repeat_start.1 as u64 - 1;
    if remaining != 0 {
        res + partial.get(&remaining).unwrap()
    } else {
        // FIXME: Investigate this off by 1 error
        res - 1
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let directions = data.split("").filter(|s| s.len() > 0).map(|s| s.chars().nth(0).unwrap()).collect::<Vec<_>>();

    let rocks = vec![vec![(0,0), (1,0), (2,0), (3,0)], vec![(0,1), (1,0), (1,1), (1,2), (2,1)], vec![(0,0), (1,0), (2,0), (2,1), (2,2)], vec![(0,0), (0,1), (0,2), (0,3)], vec![(0,0), (1,0), (0,1), (1,1)]];

    let lcm = rocks.len() * directions.len();

    let mut cave = vec![vec!['.'; lcm.max(ROCKS + 1)*3]; WIDTH];

    let mut jet = 0;
    let mut spawn_height = 3;

    let mut memory = HashMap::new();
    
    let mut repeat_start = (0,0);
    let mut repeat_end = (0,0);
    let mut remainder = HashMap::new();
    
    'outer: for i in 0..ROCKS {
        let rock = &rocks[i % rocks.len()];
        let mut position = (2, spawn_height);
        loop {
            // Jet pushing
            match directions[jet % directions.len()] {
                '>' => {
                    if !rock.iter().any(|p| position.0 + p.0 + 1 == WIDTH || cave[position.0 + p.0 + 1][position.1 + p.1] == '#') {
                        position = (position.0 + 1, position.1);
                    }
                }
                '<' => {
                    if !rock.iter().any(|p| position.0 + p.0 == 0 || cave[position.0 + p.0 - 1][position.1 + p.1] == '#') {
                        position = (position.0 - 1, position.1);
                    }
                }
                _ => unreachable!()
            }
            jet += 1;
            
            // Falling
            if rock.iter().any(|p| position.1 == 0 || cave[position.0 + p.0][position.1 + p.1 - 1] == '#') {
                // Hit bottom
                rock.iter().for_each(|rock| cave[rock.0 + position.0][rock.1 + position.1] = '#');
                spawn_height = spawn_height.max(rock.iter().map(|r| r.1 + position.1).max().unwrap() + 4);
                if spawn_height < 4 {break;}
                // Check if we are already in repeating section
                match memory.insert((i % rocks.len(), position.0, jet % directions.len()), (i, spawn_height - 3, cave.iter().map(|c| c[position.1]).collect::<String>())) {
                    Some(l) => {
                        if l.2 != cave.iter().map(|c| c[position.1]).collect::<String>() {break;}
                        // Found the point at which the pattern repeats; calculate result
                        repeat_start = (l.0, l.1);
                        repeat_end = (i, spawn_height - 3);
                        memory.iter().filter(|(_, v)| v.0 >= l.0).for_each(|(_, v)| {
                            remainder.insert(v.0 - l.0, (v.1 - l.1) as u64);
                        });
                        break 'outer;
                    }
                    None => {}
                }
                break;
            } else {
                position = (position.0, position.1 - 1);
            }
        }
    }
    
    // 3109
    let part1 = height(ROCKS as u64, repeat_start, repeat_end, &remainder);
    // 1541449275365
    let part2 = height(ITERATIONS_PART_2, repeat_start, repeat_end, &remainder);
    

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}