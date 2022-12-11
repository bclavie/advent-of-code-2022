use aoc2022::time_run2;

#[time_run2("11")]
fn main() {
    monkeys()
}

#[derive(Clone)]
struct Monkey {
    items: Vec<u64>,
    throw_number: u64,
    inspection_fn: fn(u64) -> u64,
    true_monkey_index: usize,
    false_monkey_index: usize,
    seen_items: u64,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        i: fn(u64) -> u64,
        throw_number: u64,
        true_monkey_index: usize,
        false_monkey_index: usize,
    ) -> Self {
        let items = items.into_iter().map(u64::from).collect();

        Self {
            items,
            throw_number,
            inspection_fn: i,
            true_monkey_index,
            false_monkey_index,
            seen_items: 0,
        }
    }

    fn do_inspection1(&mut self) {
        for item in self.items.iter_mut() {
            self.seen_items += 1;

            *item = (self.inspection_fn)(*item) / 3;
        }
    }

    fn do_inspection2(&mut self) {
        for item in self.items.iter_mut() {
            self.seen_items += 1;

            *item = (self.inspection_fn)(*item);
        }
    }

    fn throw_items(&mut self) -> (usize, Vec<u64>, usize, Vec<u64>) {
        let mut true_items = vec![];
        let mut false_items = vec![];

        for item in self.items.drain(..) {
            if item % self.throw_number == 0 {
                true_items.push(item)
            } else {
                false_items.push(item)
            }
        }

        (
            self.true_monkey_index,
            true_items,
            self.false_monkey_index,
            false_items,
        )
    }
}

fn monkeys() -> (String, String) {
    // could probably parse the input for this but I cba.
    let mut monkeys1: Vec<Monkey> = vec![
        Monkey::new(vec![98, 89, 52], |i| i.checked_mul(2).unwrap(), 5, 6, 1),
        Monkey::new(
            vec![57, 95, 80, 92, 57, 78],
            |i| i.checked_mul(13).unwrap(),
            2,
            2,
            6,
        ),
        Monkey::new(vec![82, 74, 97, 75, 51, 92, 83], |i| i + 5, 19, 7, 5),
        Monkey::new(vec![97, 88, 51, 68, 76], |i| i + 6, 7, 0, 4),
        Monkey::new(vec![63], |i| i + 1, 17, 0, 1),
        Monkey::new(vec![94, 91, 51, 63], |i| i + 4, 13, 4, 3),
        Monkey::new(vec![61, 54, 94, 71, 74, 68, 98, 83], |i| i + 2, 3, 2, 7),
        Monkey::new(vec![90, 56], |i| i.checked_mul(i).unwrap(), 11, 3, 5),
    ];

    let mut monkeys2 = monkeys1.clone();

    for _ in 0..20 {
        for m in 0..monkeys1.len() {
            monkeys1[m].do_inspection1();
            let (true_index, true_items, false_index, false_items) = monkeys1[m].throw_items();
            monkeys1[true_index].items.extend(true_items.into_iter());
            monkeys1[false_index].items.extend(false_items.into_iter());
        }
    }

    monkeys1.sort_by(|a, b| b.seen_items.cmp(&a.seen_items));
    let part1 = (monkeys1[0].seen_items * monkeys1[1].seen_items).to_string();

    // We can keep the numbers small by just having a modolus over the total product
    // of all the checks. Effectively our u64 are always in Mod|product|.
    let total_product: u64 = monkeys2.iter().map(|m| m.throw_number).product();

    for _ in 0..10000 {
        for m in 0..monkeys1.len() {
            for item in monkeys2[m].items.iter_mut() {
                *item %=  total_product
            }
            monkeys2[m].do_inspection2();
            let (true_index, true_items, false_index, false_items) = monkeys2[m].throw_items();
            monkeys2[true_index].items.extend(true_items.into_iter());
            monkeys2[false_index].items.extend(false_items.into_iter());
        }
    }
    monkeys2.sort_by(|a, b| b.seen_items.cmp(&a.seen_items));
    let part2 = (monkeys2[0].seen_items * monkeys2[1].seen_items).to_string();

    (part1, part2)
}
