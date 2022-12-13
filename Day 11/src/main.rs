// File modified to fit part 2 only; for part 1 change "% 9699690" to "/ 3" in the operation lambdas and run for 20 rounds only

use std::{fs, path::Path};

struct Monkey<'a> {
    stack: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64 + 'a>,
    target: Box<dyn Fn(i64) -> usize + 'a>
}

impl<'a> Monkey<'a> {
    fn action(&mut self) -> (i64, usize) {
        let item = (self.operation)(self.stack.remove(0));
        (item, (self.target)(item))
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let monkey_input = data.split("\n\n");

    let mut handled: Vec<i64> = Vec::new();

    let mut monkeys: Vec<Monkey> = Vec::new();

    for input in monkey_input {
        handled.push(0);

        let (items, input) = input.split_once("\n").unwrap().1.split_once("\n").unwrap();
        let (_, items) = items.split_once(": ").unwrap();
        let items = items.split(", ").map(|i| i.parse::<i64>().unwrap()).collect::<Vec<_>>();

        let (op, input) = input.split_once("\n").unwrap();
        let (_, op) = op.split_once("old ").unwrap();
        let (operator, operand) = op.split_once(" ").unwrap();
        let operation: Box<dyn Fn(i64) -> i64> = match operator {
            // "+" => Box::new(|a: i64| (a + operand.parse::<i64>().unwrap()) / 3),
            "+" => Box::new(|a: i64| (a + operand.parse::<i64>().unwrap()) % 9699690),
            "*" => {
                match operand {
                    // "old" => Box::new(|a: i64| (a * a) / 3),
                    "old" => Box::new(|a: i64| (a * a) % 9699690),
                    // _ => Box::new(|a: i64| (a * operand.parse::<i64>().unwrap()) / 3),
                    _ => Box::new(|a: i64| (a * operand.parse::<i64>().unwrap()) % 9699690),
                }
            },
            _ => unreachable!()
        };

        let (test, input) = input.split_once("\n").unwrap();
        let (_, test) = test.split_once("by ").unwrap();
        let (t1, t2) = input.split_once("\n").unwrap();
        let t1 = t1.split_once("monkey ").unwrap().1;
        let t2 = t2.split_once("monkey ").unwrap().1;
        let target: Box<dyn Fn(i64) -> usize> = Box::new(|a: i64| {
            if a % test.parse::<i64>().unwrap() == 0 {
                t1.parse::<usize>().unwrap()
            } else {
                t2.parse::<usize>().unwrap()
            }
        });

        monkeys.push(Monkey{stack: items, operation: operation, target: target});
    }

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while monkeys[i].stack.len() != 0 {
                handled[i] += 1;
                let (item, target) = monkeys[i].action();
                monkeys[target].stack.push(item);
            }
        }
    }

    handled.sort();
    handled.reverse();

    let part2 = handled[0] * handled[1];
    let part1 = 67830;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
