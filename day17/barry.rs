use std::{
    collections::HashMap,
    iter::{self},
};

use aoc2022::time_run2;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/17");

#[time_run2("17")]
fn main() {
    pyroclastic_flow(INPUT)
}

fn pyroclastic_flow(i: &str) -> (String, String) {
    let inputs = i.chars().map(Jet::from).collect_vec();
    let mut grid = Grid::new();

    let mut stopped_rocks = 0;
    let mut air_index = 0;
    while stopped_rocks < 2022 {
        let mut rock = grid.spawn_rock(stopped_rocks % grid.spawn_order.len());

        loop {
            let input = inputs[air_index % inputs.len()];
            air_index += 1;
            match input {
                Jet::Left => rock.move_left(&grid),
                Jet::Right => rock.move_right(&grid),
            }
            if rock.check_move_down(&grid) {
                continue;
            } else {
                grid.place_rock(rock);
                stopped_rocks += 1;
                break;
            }
        }
    }
    // The actual height is 1 more than the index
    let part1 = (grid.highest_rock_index + 1).to_string();

    let mut grid = Grid::new();
    // Basically have to find where the pattern starts to cycle.
    // We can generate a unique key using the
    // Modded Rock index, Modded Air Index of the current interation and we store
    // the iteration and height at this point in time.
    let mut state: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut stopped_rocks = 0;
    let mut air_index = 0;
    let mut part2 = 0;
    while stopped_rocks < 1000000000000 {
        let mut rock = grid.spawn_rock(stopped_rocks % grid.spawn_order.len());

        loop {
            let input = inputs[air_index % inputs.len()];
            match input {
                Jet::Left => rock.move_left(&grid),
                Jet::Right => rock.move_right(&grid),
            }
            air_index += 1;

            if rock.check_move_down(&grid) {
                continue;
            } else {
                grid.place_rock(rock);
                let highest = grid.highest_rock_index;
                stopped_rocks += 1;

                let key = (
                    stopped_rocks % grid.spawn_order.len(),
                    air_index % inputs.len(),
                );

                // On cache hit we can try grab the solution now otherwise just continue.
                if let Some((last_seen_rock, last_max_height)) = state.get(&key) {
                    let highest = grid.highest_rock_index;
                    let diff_height = grid.highest_rock_index as usize - last_max_height;

                    let repetitions = 1000000000000 - stopped_rocks;
                    let cycle_length = stopped_rocks - last_seen_rock;

                    let repeats = repetitions / cycle_length;
                    let remainder = repetitions % cycle_length;

                    // We are only really interested in the last rock - which we assume happens in
                    // a cycle and only if there is no remainder when we do the revision
                    if remainder == 0 {
                        part2 = highest as usize + diff_height * repeats;
                        // Mega scuffed way to break the loop
                        stopped_rocks = 1000000000000;
                    }
                }

                state.insert(key, (stopped_rocks, highest as usize));
                break;
            }
        }
    }

    // Again, the height is one higher than the index.
    part2 += 1;

    (part1, part2.to_string())
}

#[derive(Copy, Clone)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("unknown char"),
        }
    }
}

struct Grid {
    grid: Vec<[GridSpace; 7]>,
    highest_rock_index: i64,
    rock_has_placed: bool,
    spawn_order: Vec<RockType>,
}

impl Grid {
    fn new() -> Self {
        let grid: Vec<[GridSpace; 7]> = Default::default();
        let rocks = vec![
            RockType::HLine,
            RockType::Cross,
            RockType::LShape,
            RockType::VLine,
            RockType::Square,
        ];

        Self {
            grid,
            rock_has_placed: false,
            highest_rock_index: 0,
            spawn_order: rocks,
        }
    }

    // Spawns a rock with the left at '2' and 3 above the highest rock.
    fn spawn_rock(&mut self, i: usize) -> Rock {
        let r_type = self.spawn_order[i];

        if !self.rock_has_placed {
            match r_type {
                RockType::HLine => {
                    let left = (2, (self.highest_rock_index + 3) as usize);
                    while left.1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::HLine { left }
                }
                RockType::Cross => {
                    let centre = (3, (self.highest_rock_index + 4) as usize);
                    while centre.1 + 1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::Cross { centre }
                }
                RockType::LShape => {
                    let left = (2, (self.highest_rock_index + 3) as usize);
                    while left.1 + 2 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::LShape { left }
                }
                RockType::VLine => {
                    let top = (2, (self.highest_rock_index + 6) as usize);
                    while top.1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::VLine { top }
                }
                RockType::Square => {
                    let top_left = (2, (self.highest_rock_index + 4) as usize);
                    while top_left.1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::Square { top_left }
                }
            }
        } else {
            match r_type {
                RockType::HLine => {
                    let left = (2, (self.highest_rock_index + 4) as usize);
                    while left.1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::HLine { left }
                }
                RockType::Cross => {
                    let centre = (3, (self.highest_rock_index + 5) as usize);
                    while centre.1 + 1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::Cross { centre }
                }
                RockType::LShape => {
                    let left = (2, (self.highest_rock_index + 4) as usize);
                    while left.1 + 2 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::LShape { left }
                }
                RockType::VLine => {
                    let top = (2, (self.highest_rock_index + 7) as usize);
                    while top.1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::VLine { top }
                }
                RockType::Square => {
                    let top_left = (2, (self.highest_rock_index + 5) as usize);
                    while top_left.1 >= self.grid.len() {
                        self.extend_grid();
                    }
                    Rock::Square { top_left }
                }
            }
        }
    }

    fn place_rock(&mut self, rock: Rock) {
        match rock {
            Rock::HLine { left } => {
                debug_assert!(self.grid[left.1][left.0] == GridSpace::Air);
                debug_assert!(self.grid[left.1][left.0 + 1] == GridSpace::Air);
                debug_assert!(self.grid[left.1][left.0 + 2] == GridSpace::Air);
                debug_assert!(self.grid[left.1][left.0 + 3] == GridSpace::Air);
                self.grid[left.1][left.0] = GridSpace::Rock(RockType::HLine);
                self.grid[left.1][left.0 + 1] = GridSpace::Rock(RockType::HLine);
                self.grid[left.1][left.0 + 2] = GridSpace::Rock(RockType::HLine);
                self.grid[left.1][left.0 + 3] = GridSpace::Rock(RockType::HLine);
            }
            Rock::Cross { centre } => {
                debug_assert!(self.grid[centre.1][centre.0] == GridSpace::Air);
                debug_assert!(self.grid[centre.1][centre.0 - 1] == GridSpace::Air);
                debug_assert!(self.grid[centre.1][centre.0 + 1] == GridSpace::Air);
                debug_assert!(self.grid[centre.1 + 1][centre.0] == GridSpace::Air);
                debug_assert!(self.grid[centre.1 - 1][centre.0] == GridSpace::Air);

                self.grid[centre.1][centre.0] = GridSpace::Rock(RockType::Cross);
                self.grid[centre.1][centre.0 - 1] = GridSpace::Rock(RockType::Cross);
                self.grid[centre.1][centre.0 + 1] = GridSpace::Rock(RockType::Cross);
                self.grid[centre.1 + 1][centre.0] = GridSpace::Rock(RockType::Cross);
                self.grid[centre.1 - 1][centre.0] = GridSpace::Rock(RockType::Cross);
            }
            Rock::LShape { left } => {
                debug_assert!(self.grid[left.1][left.0] == GridSpace::Air);
                debug_assert!(self.grid[left.1][left.0 + 1] == GridSpace::Air);
                debug_assert!(self.grid[left.1][left.0 + 2] == GridSpace::Air);
                debug_assert!(self.grid[left.1 + 1][left.0 + 2] == GridSpace::Air);
                debug_assert!(self.grid[left.1 + 2][left.0 + 2] == GridSpace::Air);

                self.grid[left.1][left.0] = GridSpace::Rock(RockType::LShape);
                self.grid[left.1][left.0 + 1] = GridSpace::Rock(RockType::LShape);
                self.grid[left.1][left.0 + 2] = GridSpace::Rock(RockType::LShape);
                self.grid[left.1 + 1][left.0 + 2] = GridSpace::Rock(RockType::LShape);
                self.grid[left.1 + 2][left.0 + 2] = GridSpace::Rock(RockType::LShape);
            }
            Rock::VLine { top } => {
                debug_assert!(self.grid[top.1][top.0] == GridSpace::Air);
                debug_assert!(self.grid[top.1 - 1][top.0] == GridSpace::Air);
                debug_assert!(self.grid[top.1 - 2][top.0] == GridSpace::Air);
                debug_assert!(self.grid[top.1 - 3][top.0] == GridSpace::Air);

                self.grid[top.1][top.0] = GridSpace::Rock(RockType::VLine);
                self.grid[top.1 - 1][top.0] = GridSpace::Rock(RockType::VLine);
                self.grid[top.1 - 2][top.0] = GridSpace::Rock(RockType::VLine);
                self.grid[top.1 - 3][top.0] = GridSpace::Rock(RockType::VLine);
            }
            Rock::Square { top_left } => {
                debug_assert!(self.grid[top_left.1][top_left.0] == GridSpace::Air);
                debug_assert!(self.grid[top_left.1][top_left.0 + 1] == GridSpace::Air);
                debug_assert!(self.grid[top_left.1 - 1][top_left.0] == GridSpace::Air);
                debug_assert!(self.grid[top_left.1 - 1][top_left.0 + 1] == GridSpace::Air);

                self.grid[top_left.1][top_left.0] = GridSpace::Rock(RockType::Square);
                self.grid[top_left.1][top_left.0 + 1] = GridSpace::Rock(RockType::Square);
                self.grid[top_left.1 - 1][top_left.0] = GridSpace::Rock(RockType::Square);
                self.grid[top_left.1 - 1][top_left.0 + 1] = GridSpace::Rock(RockType::Square);
            }
        }
        self.rock_has_placed = true;

        let mut max_index = 0;
        for x in 0..self.grid[0].len() {
            for y in 0..self.grid.len() {
                if self.grid[y][x].is_rock() && y > max_index {
                    max_index = y
                }
            }
        }
        self.highest_rock_index = max_index as i64;
    }

    fn extend_grid(&mut self) {
        self.grid.extend(iter::repeat([GridSpace::Air; 7]).take(6))
    }

    #[allow(unused)]
    fn print(&self) {
        for y in (0..self.grid.len()).rev() {
            // eprint!("{:0>8} ", y);
            for x in (0..self.grid[y].len()) {
                match self.grid[y][x] {
                    GridSpace::Rock(rock_type) => match rock_type {
                        RockType::HLine => eprint!("-"),
                        RockType::Cross => eprint!("x"),
                        RockType::LShape => eprint!("L"),
                        RockType::VLine => eprint!("|"),
                        RockType::Square => eprint!("S"),
                    },
                    GridSpace::Air => eprint!("."),
                }
            }
            eprintln!();
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum GridSpace {
    Air,
    Rock(RockType),
}

impl GridSpace {
    fn is_rock(&self) -> bool {
        match self {
            GridSpace::Air => false,
            GridSpace::Rock(_) => true,
        }
    }
}

impl Default for GridSpace {
    fn default() -> Self {
        Self::Air
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum RockType {
    HLine,
    Cross,
    LShape,
    VLine,
    Square,
}

#[derive(Debug, Copy, Clone)]
enum Rock {
    HLine { left: (usize, usize) },
    Cross { centre: (usize, usize) },
    LShape { left: (usize, usize) },
    VLine { top: (usize, usize) },
    Square { top_left: (usize, usize) },
}

// DO NOT look at stuff below this line.
impl Rock {
    fn move_left(&mut self, g: &Grid) {
        match self {
            Rock::HLine { left } => {
                if left.0 == 0 || g.grid[left.1][left.0 - 1].is_rock() {
                } else {
                    *left = (left.0 - 1, left.1)
                }
            }
            Rock::Cross { centre } => {
                if centre.0 == 1
                    || g.grid[centre.1][centre.0 - 2].is_rock()
                    || g.grid[centre.1 - 1][centre.0 - 1].is_rock()
                    || g.grid[centre.1 + 1][centre.0 - 1].is_rock()
                {
                } else {
                    *centre = (centre.0 - 1, centre.1)
                }
            }
            Rock::LShape { left } => {
                if left.0 == 0
                    || g.grid[left.1][left.0 - 1].is_rock()
                    || g.grid[left.1 + 1][left.0 + 1].is_rock()
                    || g.grid[left.1 + 2][left.0 + 1].is_rock()
                {
                } else {
                    *left = (left.0 - 1, left.1)
                }
            }
            Rock::VLine { top } => {
                if top.0 == 0
                    || g.grid[top.1][top.0 - 1].is_rock()
                    || g.grid[top.1 - 1][top.0 - 1].is_rock()
                    || g.grid[top.1 - 2][top.0 - 1].is_rock()
                    || g.grid[top.1 - 3][top.0 - 1].is_rock()
                {
                } else {
                    *top = (top.0 - 1, top.1)
                }
            }
            Rock::Square { top_left } => {
                if top_left.0 == 0
                    || g.grid[top_left.1][top_left.0 - 1].is_rock()
                    || g.grid[top_left.1 - 1][top_left.0 - 1].is_rock()
                {
                } else {
                    *top_left = (top_left.0 - 1, top_left.1)
                }
            }
        }
    }

    fn move_right(&mut self, g: &Grid) {
        match self {
            Rock::HLine { left } => {
                if left.0 == 3 || g.grid[left.1][left.0 + 4].is_rock() {
                } else {
                    *left = (left.0 + 1, left.1)
                }
            }
            Rock::Cross { centre } => {
                if centre.0 == 5
                    || g.grid[centre.1][centre.0 + 2].is_rock()
                    || g.grid[centre.1 - 1][centre.0 + 1].is_rock()
                    || g.grid[centre.1 + 1][centre.0 + 1].is_rock()
                {
                } else {
                    *centre = (centre.0 + 1, centre.1)
                }
            }
            Rock::LShape { left } => {
                if left.0 == 4
                    || g.grid[left.1][left.0 + 3].is_rock()
                    || g.grid[left.1 + 1][left.0 + 3].is_rock()
                    || g.grid[left.1 + 2][left.0 + 3].is_rock()
                {
                } else {
                    *left = (left.0 + 1, left.1)
                }
            }
            Rock::VLine { top } => {
                if top.0 == 6
                    || g.grid[top.1][top.0 + 1].is_rock()
                    || g.grid[top.1 - 1][top.0 + 1].is_rock()
                    || g.grid[top.1 - 2][top.0 + 1].is_rock()
                    || g.grid[top.1 - 3][top.0 + 1].is_rock()
                {
                } else {
                    *top = (top.0 + 1, top.1)
                }
            }
            Rock::Square { top_left } => {
                if top_left.0 == 5
                    || g.grid[top_left.1][top_left.0 + 2].is_rock()
                    || g.grid[top_left.1 - 1][top_left.0 + 2].is_rock()
                {
                } else {
                    *top_left = (top_left.0 + 1, top_left.1)
                }
            }
        }
    }
    fn check_move_down(&mut self, g: &Grid) -> bool {
        match self {
            Rock::HLine { left } => {
                if left.1 == 0
                    || g.grid[left.1 - 1][left.0].is_rock()
                    || g.grid[left.1 - 1][left.0 + 1].is_rock()
                    || g.grid[left.1 - 1][left.0 + 2].is_rock()
                    || g.grid[left.1 - 1][left.0 + 3].is_rock()
                {
                    false
                } else {
                    *left = (left.0, left.1 - 1);
                    true
                }
            }
            Rock::Cross { centre } => {
                if centre.1 - 1 == 0
                    || g.grid[centre.1 - 1][centre.0 - 1].is_rock()
                    || g.grid[centre.1 - 1][centre.0 + 1].is_rock()
                    || g.grid[centre.1 - 2][centre.0].is_rock()
                {
                    false
                } else {
                    *centre = (centre.0, centre.1 - 1);
                    true
                }
            }
            Rock::LShape { left } => {
                if left.1 == 0
                    || g.grid[left.1 - 1][left.0].is_rock()
                    || g.grid[left.1 - 1][left.0 + 1].is_rock()
                    || g.grid[left.1 - 1][left.0 + 2].is_rock()
                {
                    false
                } else {
                    *left = (left.0, left.1 - 1);
                    true
                }
            }
            Rock::VLine { top } => {
                if top.1 == 3 || g.grid[top.1 - 4][top.0].is_rock() {
                    false
                } else {
                    *top = (top.0, top.1 - 1);
                    true
                }
            }
            Rock::Square { top_left } => {
                if top_left.1 == 1
                    || g.grid[top_left.1 - 2][top_left.0].is_rock()
                    || g.grid[top_left.1 - 2][top_left.0 + 1].is_rock()
                {
                    false
                } else {
                    *top_left = (top_left.0, top_left.1 - 1);
                    true
                }
            }
        }
    }
}
