use std::collections::{HashMap, HashSet, VecDeque};

use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/24");

#[time_run2("24")]
fn main() {
    blizzard_basin(INPUT)
}

fn blizzard_basin(i: &str) -> (String, String) {
    let grid = Grid::from_str(i);

    let part1 = grid.traverse(grid.entrance, grid.exit, 0);
    let time_back_to_start = grid.traverse(grid.exit, grid.entrance, part1);
    let part2 = grid.traverse(grid.entrance, grid.exit, time_back_to_start);

    (part1.to_string(), part2.to_string())
}

struct Grid {
    // Walls are x == 0, y == 0
    // exit.0
    // exit.1 + 1
    entrance: (i64, i64),
    exit: (i64, i64),
    minute_period: i64,
    // The computed positions of each blizzard at each minute.
    // We only have to work these out for minute_period == (width * height).
    // The inner hashmap is coordinates --> Blizzards occupying that coordinate.
    // The blizzards are modeled as a hashset since multiple blizzards of the same type
    // cannot occupy the same space.
    blizzards: Vec<HashMap<(i64, i64), HashSet<Blizzard>>>,
}

impl Grid {
    fn from_str(i: &str) -> Self {
        // Entrance is fixed.
        let entrance = (1, 0);
        let exit_index = i.lines().last().unwrap().find('.').unwrap();
        let exit = (exit_index as i64, i.lines().count() as i64 - 1);
        let mut blizzard_min0 = HashMap::<(i64, i64), HashSet<Blizzard>>::new();

        for (y, line) in i.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    // wall
                    '#' => continue,
                    '.' => continue,
                    '>' => {
                        let res = blizzard_min0
                            .insert((x as i64, y as i64), [Blizzard::Right].try_into().unwrap());
                        debug_assert!(res.is_none());
                    }
                    '<' => {
                        let res = blizzard_min0
                            .insert((x as i64, y as i64), [Blizzard::Left].try_into().unwrap());
                        debug_assert!(res.is_none());
                    }
                    '^' => {
                        let res = blizzard_min0
                            .insert((x as i64, y as i64), [Blizzard::Up].try_into().unwrap());
                        debug_assert!(res.is_none());
                    }
                    'v' => {
                        let res = blizzard_min0
                            .insert((x as i64, y as i64), [Blizzard::Down].try_into().unwrap());
                        debug_assert!(res.is_none());
                    }
                    _ => panic!("unexpected {}", c),
                }
            }
        }
        let width = (exit.0 - entrance.0) + 1;
        let height = ((exit.1 - 1) - (entrance.1 + 1)) + 1;
        let minute_period = width * height;

        let mut blizzards = Vec::<HashMap<(i64, i64), HashSet<Blizzard>>>::new();
        blizzards.push(blizzard_min0);

        for minute in 1..=minute_period {
            let prev_blizzard = blizzards.get((minute - 1) as usize).unwrap();
            let mut current_blizzard = HashMap::<(i64, i64), HashSet<Blizzard>>::new();
            for (coord, blizzards) in prev_blizzard {
                for bliz in blizzards {
                    match bliz {
                        Blizzard::Left => {
                            let left_coord = ((coord.0 - 2).rem_euclid(width) + 1, coord.1);
                            let mut left_blizzards = current_blizzard
                                .remove(&left_coord)
                                .or_else(|| Some(HashSet::<Blizzard>::new()))
                                .unwrap();
                            let res = left_blizzards.insert(Blizzard::Left);
                            debug_assert!(res);
                            let res = current_blizzard.insert(left_coord, left_blizzards);
                            debug_assert!(res.is_none());
                        }
                        Blizzard::Right => {
                            let right_coord = ((coord.0).rem_euclid(width) + 1, coord.1);
                            let mut right_blizzards = current_blizzard
                                .remove(&right_coord)
                                .or_else(|| Some(HashSet::<Blizzard>::new()))
                                .unwrap();
                            let res = right_blizzards.insert(Blizzard::Right);
                            debug_assert!(res);
                            let res = current_blizzard.insert(right_coord, right_blizzards);
                            debug_assert!(res.is_none());
                        }
                        Blizzard::Up => {
                            let up_coord = (coord.0, (coord.1 - 2).rem_euclid(height) + 1);
                            let mut up_blizzards = current_blizzard
                                .remove(&up_coord)
                                .or_else(|| Some(HashSet::<Blizzard>::new()))
                                .unwrap();
                            let res = up_blizzards.insert(Blizzard::Up);
                            debug_assert!(res);
                            let res = current_blizzard.insert(up_coord, up_blizzards);
                            debug_assert!(res.is_none());
                        }
                        Blizzard::Down => {
                            let down_coord = (coord.0, (coord.1).rem_euclid(height) + 1);
                            let mut down_blizzards = current_blizzard
                                .remove(&down_coord)
                                .or_else(|| Some(HashSet::<Blizzard>::new()))
                                .unwrap();
                            let res = down_blizzards.insert(Blizzard::Down);
                            debug_assert!(res);
                            let res = current_blizzard.insert(down_coord, down_blizzards);
                            debug_assert!(res.is_none());
                        }
                    }
                }
            }
            blizzards.push(current_blizzard);
        }

        Self {
            entrance,
            exit,
            minute_period,
            blizzards,
        }
    }

    fn is_in_bounds(&self, a: (i64, i64)) -> bool {
        (a == self.entrance || a == self.exit)
            || (a.0 >= self.entrance.0 && a.0 <= self.exit.0 && a.1 > 0 && a.1 < self.exit.1)
    }

    // BFS
    fn traverse(&self, start: (i64, i64), end: (i64, i64), start_mins: i64) -> i64 {
        // minute, position
        let mut queue = VecDeque::<(i64, (i64, i64))>::new();
        queue.push_back((start_mins, start));

        // If we've visited a combination of node, minute before, then we can just skip it.
        let mut visited = HashSet::<(i64, (i64, i64))>::new();

        while !queue.is_empty() {
            let (minutes, position) = queue.pop_front().unwrap();

            if visited.get(&(minutes, position)).is_some() {
                continue;
            }
            visited.insert((minutes, position));

            // If we're at the end, we are done!
            if position == end {
                return minutes;
            }

            // Get valid positions for next minute & append to queue.
            let next_blizzard_positions = self
                .blizzards
                .get(((minutes + 1) % self.minute_period) as usize)
                .unwrap();

            // Waiting
            if next_blizzard_positions.get(&position).is_none()
                && visited.get(&(minutes + 1, position)).is_none()
            {
                queue.push_back((
                    minutes + 1,
                    position,
                ))
            }

            // Up
            {
                let up = (position.0, position.1 - 1);
                if self.is_in_bounds(up)
                    && next_blizzard_positions.get(&up).is_none()
                    && visited.get(&(minutes + 1, up)).is_none()
                {
                    queue.push_back((minutes + 1, up))
                }
            }

            // Down
            {
                let down = (position.0, position.1 + 1);
                if self.is_in_bounds(down)
                    && visited.get(&(minutes + 1, down)).is_none()
                    && next_blizzard_positions.get(&down).is_none()
                {
                    queue.push_back((minutes + 1, down))
                }
            }

            // Left
            {
                let left = (position.0 - 1, position.1);
                if self.is_in_bounds(left)
                    && next_blizzard_positions.get(&left).is_none()
                    && visited.get(&(minutes + 1, left)).is_none()
                {
                    queue.push_back((minutes + 1, left))
                }
            }

            // Right
            {
                let right = (position.0 + 1, position.1);
                if self.is_in_bounds(right)
                    && next_blizzard_positions.get(&right).is_none()
                    && visited.get(&(minutes + 1, right)).is_none()
                {
                    queue.push_back((
                        minutes + 1,
                        right,
                    ))
                }
            }
        }

        panic!("path not found")
    }

    #[allow(unused)]
    fn print(&self, minute: i64, me: (i64, i64)) {
        let blizzards = self
            .blizzards
            .get((minute % self.minute_period) as usize)
            .unwrap();

        for y in 0..=self.exit.1 {
            eprint!("{:0>5} ", y);
            for x in (0..=self.exit.0 + 1) {
                let blizzard_at_this_coord = blizzards.get(&(x, y));
                if (x, y) == me {
                    eprint!("M")
                } else if (x, y) == self.entrance || (x, y) == self.exit {
                    eprint!(".")
                } else if (y == 0 || y == self.exit.1) || (x == 0 || x == self.exit.0 + 1) {
                    eprint!("#")
                } else if blizzard_at_this_coord.is_some() {
                    let blizzard_at_this_coord = blizzard_at_this_coord.unwrap();
                    match blizzard_at_this_coord.len() {
                        _ => eprint!("{}", blizzard_at_this_coord.len()),
                        1 => {
                            if blizzard_at_this_coord.get(&Blizzard::Left).is_some() {
                                eprint!("<")
                            } else if blizzard_at_this_coord.get(&Blizzard::Right).is_some() {
                                eprint!(">")
                            } else if blizzard_at_this_coord.get(&Blizzard::Up).is_some() {
                                eprint!("^")
                            } else {
                                eprint!("v")
                            }
                        }
                    }
                } else {
                    eprint!(".")
                }
            }
            eprintln!();
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Blizzard {
    Left,
    Right,
    Up,
    Down,
}
