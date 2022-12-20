use aoc2022::time_run2;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/20");

#[time_run2("20")]
fn main() {
    grove_positioning_system(INPUT)
}

fn grove_positioning_system(i: &str) -> (String, String) {
    let items = i
        .lines()
        .enumerate()
        .map(|(old_index, l)| (old_index, l.parse::<i64>().unwrap()))
        .collect_vec();
    let mut p1_answer = items.clone();
    let mut p2_answer = items.clone();

    for (old_index, _) in items.iter() {
        let (index, item) = p1_answer
            .iter()
            .enumerate()
            .find(|(_, i)| i.0 == *old_index)
            .unwrap();

        if item.1 >= 0 {
            // len - 1 swaps effectively keeps the order the same.
            for i in 0..item.1.rem_euclid(items.len() as i64 - 1) {
                let a = (index + i as usize).rem_euclid(items.len());
                let b = (index + i as usize + 1).rem_euclid(items.len());
                p1_answer.swap(a, b)
            }
        } else {
            for i in 0..item.1.abs().rem_euclid(items.len() as i64 - 1) {
                let a = (index as i64 - i).rem_euclid(items.len() as i64);
                let b = (index as i64 - i - 1).rem_euclid(items.len() as i64);

                p1_answer.swap(a as usize, b as usize)
            }
        }
    }

    let (index_of_0, _) = p1_answer
        .iter()
        .enumerate()
        .find(|(_, item)| item.1 == 0)
        .unwrap();
    let mut part1 = 0;
    for i in [1000, 2000, 3000] {
        part1 += p1_answer[(index_of_0 + i as usize).rem_euclid(items.len())].1
    }

    // Part 2 is basically just part 1 again.
    let decryption_key = 811589153;
    for item in p2_answer.iter_mut() {
        item.1 = item.1.checked_mul(decryption_key).unwrap();
    }

    for _ in 0..10 {
        for (old_index, _) in items.iter() {
            let (index, item) = p2_answer
                .iter()
                .enumerate()
                .find(|(_, i)| i.0 == *old_index)
                .unwrap();

            if item.1 >= 0 {
                // len - 1 swaps effectively keeps the order the same.
                for i in 0..item.1.rem_euclid(items.len() as i64 - 1) {
                    let a = (index + i as usize).rem_euclid(items.len());
                    let b = (index + i as usize + 1).rem_euclid(items.len());
                    p2_answer.swap(a, b)
                }
            } else {
                for i in 0..item.1.abs().rem_euclid(items.len() as i64 - 1) {
                    let a = (index as i64 - i).rem_euclid(items.len() as i64);
                    let b = (index as i64 - i - 1).rem_euclid(items.len() as i64);

                    p2_answer.swap(a as usize, b as usize)
                }
            }
        }
    }

    let (index_of_0, _) = p2_answer
        .iter()
        .enumerate()
        .find(|(_, item)| item.1 == 0)
        .unwrap();
    let mut part2 = 0;
    for i in [1000, 2000, 3000] {
        part2 += p2_answer[(index_of_0 + i as usize).rem_euclid(items.len())].1
    }

    (part1.to_string(), part2.to_string())
}
