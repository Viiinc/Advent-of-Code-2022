use std::{fs, path::Path};

fn check_increase_score(cycle: i32, reg: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        reg * cycle as i32
    } else {
        0
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut part1 = 0;
    let mut part2: Vec<Vec<char>> = Vec::new();

    for i in 0..6 {
        part2.push(Vec::new());
        for _ in 0..40 {
            part2[i].push('.');
        }
    }

    let mut cycle: i32 = 1;
    let mut register: i32 = 1;

    for line in data.lines() {
        let instr: Vec<_> = line.split_ascii_whitespace().collect();
        let mut row = ((cycle - 1) / 40) as usize;
        let mut pixel = ((cycle - 1) % 40) as usize;
        if (register - pixel as i32).abs() <= 1 {
            part2[row][pixel] = '#';
        }
        match instr[..] {
            ["noop"] => cycle += 1,
            ["addx", arg] => {
                cycle += 1;

                row = ((cycle - 1) / 40) as usize;
                pixel = (pixel + 1) % 40;
                if (register - pixel as i32).abs() <= 1 {
                    part2[row][pixel] = '#';
                }

                part1 += check_increase_score(cycle, register);

                let arg: i32 = arg.parse().unwrap();
                register += arg;
                cycle += 1;
            }
            _ => unreachable!(),
        }
        part1 += check_increase_score(cycle, register);
    }

    println!("Part 1: {},\nPart 2: \n{}", part1, part2.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
}
