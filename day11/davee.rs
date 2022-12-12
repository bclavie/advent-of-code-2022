use std::collections::VecDeque;

type Item = u64;

#[derive(Debug, Clone)]
enum Op {
    Add(u64),
    Mult(u64),
    Square,
}

impl Op {
    pub fn exec(&self, item: Item) -> Item {
        match self {
            Self::Add(num) => item.checked_add(*num).unwrap(),
            Self::Mult(num) => item.checked_mul(*num).unwrap(),
            Self::Square => item.checked_mul(item).unwrap(),
        }
    }
}

impl From<&str> for Op {
    fn from(input: &str) -> Self {
        let equation = input.split_once("new = ").map(|(_, x)| x).unwrap();

        if equation == "old * old" {
            Self::Square
        } else if let Some((_, num)) = equation.split_once("old + ") {
            Self::Add(num.parse().unwrap())
        } else if let Some((_, num)) = equation.split_once("old * ") {
            Self::Mult(num.parse().unwrap())
        } else {
            panic!("unknown test")
        }
    }
}

#[derive(Debug, Clone)]
struct Test(u64);

impl Test {
    pub fn test(&self, item: Item) -> bool {
        item % self.0 == 0
    }
}

impl From<&str> for Test {
    fn from(input: &str) -> Self {
        if let Some((_, num)) = input.split_once("divisible by ") {
            Self(num.parse().unwrap())
        } else {
            panic!("unknown test")
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<Item>,
    operation: Op,
    test: Test,
    on_true: usize,
    on_false: usize,
    inspections: usize,
}

enum Worried {
    Worried,
    NotWorried(u64),
}

impl Monkey {
    pub fn inspect(&mut self, worry: Worried) -> Option<(Item, usize)> {
        let item = self.items.pop_front()?;
        let item = match worry {
            Worried::Worried => self.operation.exec(item) / 3,
            Worried::NotWorried(lcm) => self.operation.exec(item) % lcm,
        };

        self.inspections += 1;

        if self.test.test(item) {
            Some((item, self.on_true))
        } else {
            Some((item, self.on_false))
        }
    }

    pub fn thrown(&mut self, item: Item) {
        self.items.push_back(item);
    }
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let items = input
            .split_once("Starting items:")
            .map(|(_, x)| x.lines().next().unwrap())
            .expect("no items")
            .trim();
        let operation = input
            .split_once("Operation:")
            .map(|(_, x)| x.lines().next().unwrap())
            .expect("no operation")
            .trim();
        let test = input
            .split_once("Test:")
            .map(|(_, x)| x.lines().next().unwrap())
            .expect("no test")
            .trim();
        let on_true = input
            .split_once("If true:")
            .map(|(_, x)| x.lines().next().unwrap())
            .expect("no on true")
            .trim();
        let on_false = input
            .split_once("If false:")
            .map(|(_, x)| x.lines().next().unwrap())
            .expect("no on false")
            .trim();

        Self {
            items: items
                .split(',')
                .map(|x| x.trim().parse::<Item>().unwrap())
                .collect(),
            operation: operation.into(),
            test: test.into(),
            on_true: on_true.split(' ').last().unwrap().parse().unwrap(),
            on_false: on_false.split(' ').last().unwrap().parse().unwrap(),
            inspections: 0,
        }
    }
}

fn part1(mut monkeys: Vec<Monkey>) {
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            while let Some((item, to)) = monkeys[monkey].inspect(Worried::Worried) {
                monkeys[to].thrown(item);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();
    inspections.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    println!("p1: {}", inspections[0] * inspections[1]);
}

fn lcm(x: u64, y: u64) -> u64 {
    x.checked_mul(y).unwrap() / gcd::binary_u64(x, y)
}

fn part2(mut monkeys: Vec<Monkey>) {
    // looks like they are all prime... but maybe i got lucky?
    let lcm = monkeys.iter().fold(1, |acc, x| lcm(acc, x.test.0));

    for _ in 0..10000 {
        for monkey in 0..monkeys.len() {
            while let Some((item, to)) = monkeys[monkey].inspect(Worried::NotWorried(lcm)) {
                monkeys[to].thrown(item);
            }
        }
    }

    let mut inspections = monkeys.iter().map(|x| x.inspections).collect::<Vec<_>>();
    inspections.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    println!("p2: {}", inspections[0] * inspections[1]);
}

fn main() {
    let input = include_str!("../input.txt");

    let monkeys = input.split("\n\n").map(Monkey::from).collect::<Vec<_>>();
    part1(monkeys.clone());
    part2(monkeys);
}
