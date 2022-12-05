use aoc2022::time_run;

const INPUT: &str = include_str!("../inputs/05");

#[time_run("05")]
fn main() {
    supply_stacks(INPUT)
}

struct CrateGrid {
    inner: [Vec<char>; 9],
}

impl CrateGrid {
    fn new_from_str(s: &str) -> Self {
        let crate_grid: Vec<Vec<char>> = s
            .lines()
            .map(|line| {
                let chars: Vec<char> = line.chars().collect();
                chars
            })
            .collect();

        let mut inner: [Vec<char>; 9] = Default::default();

        // Input only goes up to 9 columns, so interating over `char` here is fine.
        let last = crate_grid.last().unwrap();

        for (j, _) in last.iter().enumerate() {
            if let Ok(crate_number) = last[j].to_string().parse::<u8>() {
                for i in (0..crate_grid.len() - 1).rev() {
                    if crate_grid[i][j] != ' ' {
                        inner[(crate_number - 1) as usize].push(crate_grid[i][j])
                    }
                }
            }
        }

        Self { inner }
    }

    #[allow(unused)]
    fn do_move(&mut self, m: Move) {
        for _ in 0..m.amount {
            let c = self.inner[m.from_index as usize].pop().unwrap();
            self.inner[m.to_index as usize].push(c)
        }
    }

    fn do_move2(&mut self, m: Move) {
        let current_length = self.inner[m.from_index].len();

        let crates: Vec<char> = self.inner[m.from_index]
            .drain((current_length - m.amount)..current_length)
            .collect();
        for c in crates {
            self.inner[m.to_index].push(c);
        }
    }
}

struct Move {
    amount: usize,
    from_index: usize,
    to_index: usize,
}

impl Move {
    fn from_str(s: &str) -> Self {
        let (_, s) = s.split_at(5);
        let (amount, s) = s.split_once(" from ").unwrap();
        let (from, to) = s.split_once(" to ").unwrap();

        Self {
            amount: amount.parse().unwrap(),
            from_index: from.parse::<usize>().unwrap() - 1,
            to_index: to.parse::<usize>().unwrap() - 1,
        }
    }
}

fn supply_stacks(i: &str) -> String {
    let (starting_positions_input, movements_input) = i.split_once("\n\n").unwrap();

    let mut grid = CrateGrid::new_from_str(starting_positions_input);

    for m in movements_input.lines().map(Move::from_str) {
        // Part 1
        // grid.do_move(m)

        // Part 2
        grid.do_move2(m)
    }

    let mut solution = String::default();
    for i in grid.inner {
        solution.push(*i.last().unwrap())
    }

    solution
}
