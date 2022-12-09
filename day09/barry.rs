use std::collections::HashSet;

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/09");

#[time_run2("09")]
fn main() {
    bridges_and_stuff(INPUT)
}

enum Move {
    Up,
    Down,
    Left,
    Right,
}

impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "R" => Self::Right,
            "U" => Self::Up,
            "L" => Self::Left,
            "D" => Self::Down,
            _ => panic!("unexpected move type"),
        }
    }
}

fn bridges_and_stuff(i: &str) -> (String, String) {
    let mut visited_tail_locations_1: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_tail_locations_2: HashSet<(i64, i64)> = HashSet::new();
    visited_tail_locations_1.insert((0, 0));
    visited_tail_locations_2.insert((0, 0));

    let mut knot_positions = [(0, 0); 10];

    for line in i.lines() {
        let (move_type, amount) = line
            .split_once(" ")
            .map(|l| (Move::from_str(l.0), l.1.parse::<usize>().unwrap()))
            .unwrap();

        for _ in 0..amount {
            match move_type {
                Move::Up => knot_positions[0] = (knot_positions[0].0, knot_positions[0].1 + 1),
                Move::Down => knot_positions[0] = (knot_positions[0].0, knot_positions[0].1 - 1),
                Move::Left => knot_positions[0] = (knot_positions[0].0 - 1, knot_positions[0].1),
                Move::Right => knot_positions[0] = (knot_positions[0].0 + 1, knot_positions[0].1),
            }

            for k in 1..10 {
                knot_positions[k] = adjust_tail_position(knot_positions[k - 1], knot_positions[k]);
            }
            visited_tail_locations_1.insert(knot_positions[1]);
            visited_tail_locations_2.insert(knot_positions[9]);
        }
    }

    let part1 = visited_tail_locations_1.len().to_string();
    let part2 = visited_tail_locations_2.len().to_string();

    (part1, part2)
}

fn adjust_tail_position(head: (i64, i64), tail: (i64, i64)) -> (i64, i64) {
    let x_diff = head.0 - tail.0;
    let y_diff = head.1 - tail.1;

    // Special case for Part 2 - we can end up in a situation where we move in full diagonals.
    // E.g. for `3`, H moves to the left.
    // ..H1..
    // ....2.
    // .....3
    //
    // .H12..
    // ....3.
    // ......
    if (x_diff.abs() == 2) && (y_diff.abs() == 2) {
        return (tail.0 + x_diff.signum(), tail.1 + y_diff.signum());
    }
    // touching.
    else if (x_diff.abs() <= 1) && (y_diff.abs() <= 1) {
        return tail;
    // Horsey based movement.
    } else if (x_diff.abs() == 1) && (y_diff.abs() == 2)
        || (x_diff.abs() == 2) & (y_diff.abs() == 1)
    {
        return (tail.0 + x_diff.signum(), tail.1 + y_diff.signum());
    }
    // Horizontals
    else if (x_diff.abs() == 2) && (y_diff == 0) {
        return (tail.0 + x_diff.signum(), tail.1);
    // Verticals
    } else if (x_diff.abs() == 0) && (y_diff.abs() == 2) {
        return (tail.0, tail.1 + y_diff.signum());
    } else {
        panic!("unexpected difference: {} {}", x_diff, y_diff)
    }
}
