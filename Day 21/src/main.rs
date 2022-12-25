use std::{fs, path::Path, collections::HashMap};

use regex::Regex;

// const KEY: String = "root".to_string();

#[derive(PartialEq, Eq, Clone)]
struct Monkey {
    name: String,
    arg1: String,
    arg2: String,
    operand: char
}

fn print_equation(root: &Monkey, monkeys: &HashMap<String,&Monkey>, numbers: &HashMap<String, f64>) -> String {
    if root.name == "humn" {
        return "x".to_string();
    }
    let left: String;
    if numbers.contains_key(&root.arg1) {
        left = numbers.get(&root.arg1).unwrap().to_string();
    } else {
        let next = monkeys.get(&root.arg1).unwrap();
        left = print_equation(next, monkeys, numbers);
    }
    let right: String;
    if numbers.contains_key(&root.arg2) {
        right = numbers.get(&root.arg2).unwrap().to_string();
    } else {
        let next = monkeys.get(&root.arg2).unwrap();
        right = print_equation(next, monkeys, numbers);
    }
    return "(".to_string() + &left + &root.operand.to_string() + &right + &")".to_string();
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
      
    let re = Regex::new(r"([a-z]{4}): (-?\d+)").unwrap();

    let mut numbers = HashMap::new();

    for cap in re.captures_iter(&data) {
        numbers.insert(cap[1].to_string(), cap[2].parse::<f64>().unwrap());
    }

    let mut original_numbers = numbers.clone();
    
    let re = Regex::new(r"([a-z]{4}): ([a-z]{4}) (.{1}) ([a-z]{4})").unwrap();

    let mut operations = vec![];

    for cap in re.captures_iter(&data) {
        let monkey = cap[1].to_string();
        let arg1 = cap[2].parse().unwrap();
        let arg2 = cap[4].parse().unwrap();
        let operand = cap[3].chars().nth(0).unwrap();
        operations.push(Monkey {name: monkey, operand: operand, arg1: arg1, arg2: arg2});
    }

    while !numbers.contains_key("root") {
        operations.iter().for_each(|m| {
            let arg1 = numbers.get(&m.arg1);
            let arg2 = numbers.get(&m.arg2);
            if arg1.is_some() && arg2.is_some() {
                match m.operand {
                    '+' => {numbers.insert(m.name.clone(), arg1.unwrap() + arg2.unwrap());},
                    '/' => {numbers.insert(m.name.clone(), arg1.unwrap() / arg2.unwrap());},
                    '-' => {numbers.insert(m.name.clone(), arg1.unwrap() - arg2.unwrap());},
                    '*' => {numbers.insert(m.name.clone(), arg1.unwrap() * arg2.unwrap());},
                    _ => unreachable!()
                }
            } else {
                // Should probably filter out those we've been able to fill, but eh
            }
        });
    }
    
    let part1 = numbers.get("root").unwrap();
    
    original_numbers.remove(&"humn".to_string());
    for i in 0..operations.len() {
        if operations[i].name == "root" {
            operations[i].operand = '=';
        }
    }
    
    loop {
        let count = operations.len();
        operations.iter().for_each(|m| {
            let arg1 = original_numbers.get(&m.arg1);
            let arg2 = original_numbers.get(&m.arg2);
            if arg1.is_some() && arg2.is_some() {
                match m.operand {
                    '+' => {original_numbers.insert(m.name.clone(), arg1.unwrap() + arg2.unwrap());},
                    '/' => {original_numbers.insert(m.name.clone(), arg1.unwrap() / arg2.unwrap());},
                    '-' => {original_numbers.insert(m.name.clone(), arg1.unwrap() - arg2.unwrap());},
                    '*' => {original_numbers.insert(m.name.clone(), arg1.unwrap() * arg2.unwrap());},
                    _ => unreachable!()
                }
            }
        });
        operations = operations.iter().filter(|m| !original_numbers.contains_key(&m.name)).map(|m| m.clone()).collect::<Vec<_>>();
        if count == operations.len() {
            break;
        }
    }

    let mut monkeys = HashMap::new();

    operations.iter().for_each(|m| {
        monkeys.insert(m.name.clone(), m);
    });

    let root = *monkeys.get("root").unwrap();
    let mut next: String;
    let mut counter: f64;
    if original_numbers.contains_key(&root.arg1) {
        counter = *original_numbers.get(&root.arg1).unwrap();
        next = root.arg2.clone();
    } else {
        counter = *original_numbers.get(&root.arg2).unwrap();
        next = root.arg1.clone();
    }

    loop {
        if next == "humn".to_string() {break;}
        let operation = monkeys.get(&next).unwrap();
        if original_numbers.contains_key(&operation.arg1) {
            let arg = original_numbers.get(&operation.arg1).unwrap();
            match operation.operand {
                '+' => {counter -= arg;}
                '-' => {counter += arg;}
                '/' => {counter *= arg;}
                '*' => {counter /= arg;}
                _ => unreachable!()
            }
            next = operation.arg2.clone();
        } else {
            let arg = original_numbers.get(&operation.arg2).unwrap();
            match operation.operand {
                '+' => {counter -= arg;}
                '-' => {counter += arg;}
                '/' => {counter *= arg;}
                '*' => {counter /= arg;}
                _ => unreachable!()
            }
            next = operation.arg1.clone();

        }
    }
    let humn = Monkey { name: "humn".to_string(), arg1: "".to_string(), arg2: "".to_string(), operand: ' ' };
    monkeys.insert("humn".to_string(), &humn);

    // Easy solution; enter this into wolfram alpha
    let _sol = print_equation(root, &monkeys, &original_numbers);

    // Should be 3093175982595, doesn't work for some reason (used wolfram alpha instead)
    let part2 = counter;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}