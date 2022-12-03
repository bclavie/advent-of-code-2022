use itertools::Itertools;
use aoc2022::time_run;

const INPUT: &str = include_str!("../inputs/03");

#[time_run("03")]
fn main() {
    priority_sum(INPUT)
}


// part 1
fn common_item_sum(input: &str) -> String {
    let mut total = 0;
    for rucksack in input.split("\n") {
        let (first, last) = rucksack.split_at(rucksack.len()/2);

        for char in first.chars() {
            if last.contains(char) {
                total += item_value(char);
                break;
            }
        }
    }

    total.to_string()
}

// part 2
fn priority_sum (input: &str) -> String {
    let mut total = 0;
    let rucksacks: Vec<&str> = input.split("\n").collect();
    for group in rucksacks.into_iter().tuples::<(_,_,_)>() {
        // probably not the most efficient way but w/e.
        for c in group.0.chars() {
            if group.1.chars().contains(&c) && group.2.chars().contains(&c) {
                total += item_value(c);
                break;
            }
        }
    }

    total.to_string()
}

fn item_value(c: char) -> u32 {
    // ` is 1 below 'a' on the ascii table. Saves having to write a huge match statement on singular
    // letters.
    if c.is_ascii_lowercase() {
        return c as u32 - '`' as u32;
    }

    if c.is_ascii_uppercase(){
        return 26 + c as u32 - '@' as u32;
    }

    panic!("{} is not ascii lower or upper", c)
}
