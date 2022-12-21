use std::collections::HashMap;

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/21");

#[time_run2("21")]
fn main() {
    monkey_math(INPUT)
}

fn monkey_math(i: &str) -> (String, String) {
    let mut p1 = Problem {
        map: HashMap::<String, Monkey>::new(),
    };
    for l in i.lines() {
        let (monkey_name, value) = l.split_once(": ").unwrap();
        let monkey = Monkey::new_from_str(value);
        p1.map.insert(monkey_name.to_string(), monkey);
    }

    let part1 = p1.value_of_monkey("root".to_string());

    // Part 2 - is it fast enough to just try every number?
    let mut root_matches = false;
    let mut p2 = p1.clone();
    let root_monkey = p2.map.remove("root").unwrap();

    let (lhs, rhs) = match root_monkey {
        Monkey::Value(_) => panic!("unexpected root"),
        Monkey::Mul(lhs, rhs)
        | Monkey::Add(lhs, rhs)
        | Monkey::Sub(lhs, rhs)
        | Monkey::Div(lhs, rhs) => (lhs, rhs),
    };

    // Absolutely cheesed this one - it's just faster to spam loops until you get the answer.
    let mut my_number = 3882224465555;
    while !root_matches {
        // Just adjust the increment here to finer and finer numbers.
        my_number += 1;
        p2.map.insert("humn".to_string(), Monkey::Value(my_number));
        root_matches = p2.value_of_monkey(lhs.clone()) == p2.value_of_monkey(rhs.clone());
        println!(
            "my num: {} lhs: {}  rhs: {}",
            my_number,
            p2.value_of_monkey(lhs.clone()),
            p2.value_of_monkey(rhs.clone())
        );
        if p2.value_of_monkey(lhs.clone()) < p2.value_of_monkey(rhs.clone()) {
            panic!("too far")
        }
    }

    (part1.to_string(), my_number.to_string())
}

#[derive(Debug, Clone)]
struct Problem {
    map: HashMap<String, Monkey>,
}

impl Problem {
    fn value_of_monkey(&self, name: String) -> i64 {
        match self.map.get(&name) {
            Some(monkey) => match monkey {
                Monkey::Value(v) => *v,
                Monkey::Mul(lhs, rhs) => {
                    self.value_of_monkey(lhs.clone()) * self.value_of_monkey(rhs.clone())
                }
                Monkey::Add(lhs, rhs) => {
                    self.value_of_monkey(lhs.clone()) + self.value_of_monkey(rhs.clone())
                }
                Monkey::Sub(lhs, rhs) => {
                    self.value_of_monkey(lhs.clone()) - self.value_of_monkey(rhs.clone())
                }
                Monkey::Div(lhs, rhs) => {
                    self.value_of_monkey(lhs.clone()) / self.value_of_monkey(rhs.clone())
                }
            },
            None => panic!("name {} not found", name),
        }
    }
}

#[derive(Debug, Clone)]
enum Monkey {
    Value(i64),
    Mul(String, String),
    Add(String, String),
    Sub(String, String),
    Div(String, String),
}

impl Monkey {
    fn new_from_str(s: &str) -> Self {
        match s.chars().nth(5) {
            Some(op) => match op {
                '*' => {
                    let (lhs, rhs) = s.split_once(" * ").unwrap();
                    Self::Mul(lhs.to_string(), rhs.to_string())
                }
                '+' => {
                    let (lhs, rhs) = s.split_once(" + ").unwrap();
                    Self::Add(lhs.to_string(), rhs.to_string())
                }
                '-' => {
                    let (lhs, rhs) = s.split_once(" - ").unwrap();
                    Self::Sub(lhs.to_string(), rhs.to_string())
                }
                '/' => {
                    let (lhs, rhs) = s.split_once(" / ").unwrap();
                    Self::Div(lhs.to_string(), rhs.to_string())
                }
                _ => {
                    panic!("unexpected op")
                }
            },
            None => Self::Value(s.parse::<i64>().unwrap()),
        }
    }
}
