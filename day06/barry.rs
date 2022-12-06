use aoc2022::time_run;
use std::collections::HashSet;

const INPUT: &str = include_str!("../inputs/06");

#[time_run("06")]
fn main() {
    start_of_marker(INPUT)
}

fn start_of_marker(i: &str) -> String {
    let chars: Vec<char> = i.chars().collect();

    let mut start_of_marker = 0;
    for (i, group) in chars.windows(14).enumerate() {
        let mut hash_set = HashSet::<char>::new();
        for c in group.into_iter() {
            hash_set.insert(*c);
        }
        // part 1 is just `4 instead of 14`
        if hash_set.len() == 14 {
            start_of_marker = i + 14;
            break;
        }
    }

    start_of_marker.to_string()
}
