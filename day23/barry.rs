use std::collections::{HashMap, HashSet};

use aoc2022::time_run2;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/23");

#[time_run2("23")]
fn main() {
    unstable_diffusion(INPUT)
}

fn unstable_diffusion(i: &str) -> (String, String) {
    let mut grid = Grid::from_str(i);

    let mut directions = [
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ];

    let mut part2 = 0;
    let mut part1 = 0;
    for i in 1..100000 {
        grid.first_half(directions);
        let elves_moved = grid.second_half();
        let new_directions = [directions[1], directions[2], directions[3], directions[0]];
        directions = new_directions;

        if i == 10 {
            part1 = grid.ground_tiles();
        }

        if elves_moved == 0 {
            part2 = i;
            break;
        }
    }

    (part1.to_string(), part2.to_string())
}

#[derive(Debug, Clone)]
struct Grid {
    elves: HashSet<(i64, i64)>,
    // Proposed Location / Location of Elf that is going there.
    proposed: HashMap<(i64, i64), (i64, i64)>,
    // Proposed Location / How many Elves that are going there.
    count: HashMap<(i64, i64), u64>,
}

impl Grid {
    fn from_str(i: &str) -> Self {
        let mut elves = HashSet::<(i64, i64)>::new();

        // Note that North is smaller on the Y
        // West is smaller on the X
        for (y, line) in i.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    elves.insert((x as i64, y as i64));
                }
            }
        }

        Self {
            elves,
            proposed: HashMap::new(),
            count: HashMap::new(),
        }
    }

    // Returns true if the proposal is successful (i.e. the square is not occupied.
    fn propose_movement_to_coord(
        &mut self,
        elf_coord: (i64, i64),
        proposed: (i64, i64),
        check_coords: [(i64, i64); 3],
    ) -> bool {
        // Check if squares are empty
        if self.elves.get(&check_coords[0]).is_none()
            && self.elves.get(&check_coords[1]).is_none()
            && self.elves.get(&check_coords[2]).is_none()
        {
            match self.count.get(&proposed) {
                // If someone already proposed, then increment the value.
                Some(i) => {
                    self.count.insert(proposed, i + 1);
                }
                // Otherwise store the elf's current value in proposed.
                None => {
                    let res = self.proposed.insert(proposed, elf_coord);

                    debug_assert!(res.is_none());
                    let res = self.count.insert(proposed, 1);
                    debug_assert!(res.is_none());
                }
            };
            true
        } else {
            false
        }
    }

    fn first_half(&mut self, directions: [Direction; 4]) {
        self.proposed.clear();
        self.count.clear();
        let elves = self.elves.clone().into_iter().collect_vec();

        for elf_coord in &elves {
            // First check if the elf has any neighbours - if not don't do anything.
            if self
                .elves
                .get(&(elf_coord.0 - 1, elf_coord.1 - 1))
                .is_none()
                && self
                    .elves
                    .get(&(elf_coord.0 - 1, elf_coord.1 + 1))
                    .is_none()
                && self.elves.get(&(elf_coord.0, elf_coord.1 - 1)).is_none()
                && self.elves.get(&(elf_coord.0, elf_coord.1 + 1)).is_none()
                && self
                    .elves
                    .get(&(elf_coord.0 + 1, elf_coord.1 - 1))
                    .is_none()
                && self
                    .elves
                    .get(&(elf_coord.0 + 1, elf_coord.1 + 1))
                    .is_none()
                && self.elves.get(&(elf_coord.0 - 1, elf_coord.1)).is_none()
                && self.elves.get(&(elf_coord.0 + 1, elf_coord.1)).is_none()
            {
                continue;
            }

            for direction in directions {
                match direction {
                    Direction::North => {
                        if self.propose_movement_to_coord(
                            *elf_coord,
                            (elf_coord.0, elf_coord.1 - 1),
                            [
                                (elf_coord.0, elf_coord.1 - 1),
                                (elf_coord.0 - 1, elf_coord.1 - 1),
                                (elf_coord.0 + 1, elf_coord.1 - 1),
                            ],
                        ) {
                            break;
                        }
                    }
                    Direction::South => {
                        if self.propose_movement_to_coord(
                            *elf_coord,
                            (elf_coord.0, elf_coord.1 + 1),
                            [
                                (elf_coord.0, elf_coord.1 + 1),
                                (elf_coord.0 - 1, elf_coord.1 + 1),
                                (elf_coord.0 + 1, elf_coord.1 + 1),
                            ],
                        ) {
                            break;
                        }
                    }
                    Direction::West => {
                        if self.propose_movement_to_coord(
                            *elf_coord,
                            (elf_coord.0 - 1, elf_coord.1),
                            [
                                (elf_coord.0 - 1, elf_coord.1),
                                (elf_coord.0 - 1, elf_coord.1 - 1),
                                (elf_coord.0 - 1, elf_coord.1 + 1),
                            ],
                        ) {
                            break;
                        }
                    }
                    Direction::East => {
                        if self.propose_movement_to_coord(
                            *elf_coord,
                            (elf_coord.0 + 1, elf_coord.1),
                            [
                                (elf_coord.0 + 1, elf_coord.1),
                                (elf_coord.0 + 1, elf_coord.1 - 1),
                                (elf_coord.0 + 1, elf_coord.1 + 1),
                            ],
                        ) {
                            break;
                        }
                    }
                }
            }
        }
    }

    fn second_half(&mut self) -> u64 {
        let mut elves_moved = 0;
        for (target, elf_coord) in self.proposed.iter() {
            match self.count.get(target) {
                Some(i) if *i == 1 => {
                    elves_moved += 1;
                    let res = self.elves.remove(elf_coord);
                    debug_assert!(res);
                    let res = self.elves.insert(*target);
                    debug_assert!(res);
                }
                _ => {
                    // Either no movement or too many elves wanted to move here so do nothing.
                }
            }
        }
        elves_moved
    }

    fn ground_tiles(&self) -> u64 {
        let max_y = self.elves.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let min_y = self.elves.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let max_x = self.elves.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap();
        let min_x = self.elves.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();

        let y_diff = (max_y.1 - min_y.1) as u64 + 1;
        let x_diff = (max_x.0 - min_x.0) as u64 + 1;
        let total_area = x_diff * y_diff;

        let elves = self.elves.len();

        total_area - elves as u64
    }

    #[allow(unused)]
    fn print(&self) {
        let max_y = self.elves.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let min_y = self.elves.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        let max_x = self.elves.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap();
        let min_x = self.elves.iter().min_by(|a, b| a.0.cmp(&b.0)).unwrap();

        for y in min_y.1..=max_y.1 {
            eprint!("{:0>5} ", y);
            for x in (min_x.0..=max_x.0) {
                match self.elves.get(&(x, y)) {
                    Some(_) => eprint!("#"),
                    None => eprint!("."),
                }
            }
            eprintln!();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}
