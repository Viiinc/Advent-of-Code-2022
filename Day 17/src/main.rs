use std::{fs, path::Path};
       
const ROCKS: usize = 2022;
const WIDTH: usize = 7;
const ITERATIONS_PART_2: u64 = 1000000000000;

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let directions = data.split("").filter(|s| s.len() > 0).map(|s| s.chars().nth(0).unwrap()).collect::<Vec<_>>();

    let rocks = vec![vec![(0,0), (1,0), (2,0), (3,0)], vec![(0,1), (1,0), (1,1), (1,2), (2,1)], vec![(0,0), (1,0), (2,0), (2,1), (2,2)], vec![(0,0), (0,1), (0,2), (0,3)], vec![(0,0), (1,0), (0,1), (1,1)]];

    let mut cave = vec![vec!['.'; ROCKS*3]; WIDTH];

    let mut jet = 0;
    let mut spawn_height = 3;

    for i in 0..ROCKS {
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
                // Hit bottom, continue
                rock.iter().for_each(|rock| cave[rock.0 + position.0][rock.1 + position.1] = '#');
                spawn_height = spawn_height.max(rock.iter().map(|r| r.1 + position.1).max().unwrap() + 4);
                break;
            } else {
                position = (position.0, position.1 - 1);
            }
        }
    }

    // > 2828
    let part1 = spawn_height - 3;
    let part2 = 0;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}