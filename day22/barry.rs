use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/22");

#[time_run2("22")]
fn main() {
    monkey_map(INPUT)
}

fn monkey_map(i: &str) -> (String, String) {
    let (map, instruction_str) = i.split_once("\n\n").unwrap();
    let mut grid = Grid::from_str(map);

    let mut instructions: Vec<Instruction> = vec![];
    let mut last = 0;
    for (index, lr) in instruction_str.match_indices(|c| c == 'L' || c == 'R') {
        if last != index {
            instructions.push(Instruction::Move(
                instruction_str[last..index].parse::<usize>().unwrap(),
            ));
        }
        match lr {
            "L" => instructions.push(Instruction::RotateLeft),
            "R" => instructions.push(Instruction::RotateRight),
            _ => panic!("unexpected {}", lr),
        }
        last = index + lr.len();
    }
    if last < instruction_str.len() {
        instructions.push(Instruction::Move(
            instruction_str[last..].parse::<usize>().unwrap(),
        ));
    }
    grid.set_starting_point();
    let mut grid2 = grid.clone();

    for i in instructions.clone() {
        grid.do_instruction(i);
    }

    for i in instructions {
        grid2.do_instruction_cube(i);
    }

    (grid.get_score().to_string(), grid2.get_score().to_string())
}

#[derive(Debug, Clone)]
struct Grid {
    // Y, X
    map: Vec<Vec<Square>>,
    my_location: (usize, usize),
    my_facing: Facing,
}

impl Grid {
    fn from_str(i: &str) -> Self {
        let mut map: Vec<Vec<Square>> = vec![];
        for line in i.lines() {
            let mut things: Vec<Square> = vec![];
            for c in line.chars() {
                match c {
                    ' ' => things.push(Square::None),
                    '#' => things.push(Square::Wall),
                    '.' => things.push(Square::Open),
                    _ => panic!("unexpected char {}", c),
                }
            }
            map.push(things);
        }
        // Pad out the grid.
        let max_x = map.iter().map(|x| x.len()).max().unwrap();
        for x in map.iter_mut() {
            if x.len() < max_x {
                for _ in 0..max_x - x.len() {
                    x.push(Square::None)
                }
            }
        }

        Self {
            map,
            my_location: (0, 0),
            my_facing: Facing::Right,
        }
    }

    fn set_starting_point(&mut self) {
        let mut starting_x = 0;
        for (i, sq) in self.map[0].iter().enumerate() {
            if sq == &Square::Open {
                starting_x = i;
                break;
            }
        }

        self.map[0][starting_x] = Square::Me(Facing::Right);
        self.my_location = (starting_x, 0);
    }

    fn do_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(num_move) => match self.my_facing {
                Facing::Left => {
                    let mut x_range: Vec<usize> = (self.my_location.0..self.map[0].len()).collect();
                    x_range.extend::<Vec<usize>>((0..self.my_location.0).collect());

                    let mut moved_squares = 0;
                    for x in x_range.iter().rev() {
                        match self.map[self.my_location.1][*x] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Left);
                                self.map[self.my_location.1][*x] = Square::Me(Facing::Left);
                                self.my_location = (*x, self.my_location.1);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
                Facing::Right => {
                    let mut x_range: Vec<usize> =
                        (self.my_location.0 + 1..self.map[0].len()).collect();
                    x_range.extend::<Vec<usize>>((0..=self.my_location.0).collect());

                    let mut moved_squares = 0;
                    for x in x_range.iter() {
                        match self.map[self.my_location.1][*x] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Right);
                                self.map[self.my_location.1][*x] = Square::Me(Facing::Right);
                                self.my_location = (*x, self.my_location.1);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
                Facing::Up => {
                    let mut y_range: Vec<usize> = (self.my_location.1..self.map.len()).collect();
                    y_range.extend::<Vec<usize>>((0..self.my_location.1).collect());

                    let mut moved_squares = 0;
                    for y in y_range.iter().rev() {
                        match self.map[*y][self.my_location.0] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Up);
                                self.map[*y][self.my_location.0] = Square::Me(Facing::Up);
                                self.my_location = (self.my_location.0, *y);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
                Facing::Down => {
                    let mut y_range: Vec<usize> =
                        (self.my_location.1 + 1..self.map.len()).collect();
                    y_range.extend::<Vec<usize>>((0..=self.my_location.1).collect());

                    let mut moved_squares = 0;
                    for y in y_range.iter() {
                        match self.map[*y][self.my_location.0] {
                            Square::Me(_) => {
                                self.print();
                                panic!("unexpected me!")
                            }
                            // Move self.
                            Square::Path(_) | Square::Open => {
                                self.map[self.my_location.1][self.my_location.0] =
                                    Square::Path(Facing::Down);
                                self.map[*y][self.my_location.0] = Square::Me(Facing::Down);
                                self.my_location = (self.my_location.0, *y);
                                moved_squares += 1;
                                if moved_squares == num_move {
                                    break;
                                }
                            }
                            // Stop
                            Square::Wall => {
                                break;
                            }
                            Square::None => continue,
                        }
                    }
                }
            },
            Instruction::RotateRight => match self.my_facing {
                Facing::Left => {
                    self.my_facing = Facing::Up;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Up);
                }
                Facing::Right => {
                    self.my_facing = Facing::Down;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Down);
                }
                Facing::Up => {
                    self.my_facing = Facing::Right;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Right);
                }
                Facing::Down => {
                    self.my_facing = Facing::Left;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Left);
                }
            },
            Instruction::RotateLeft => match self.my_facing {
                Facing::Left => {
                    self.my_facing = Facing::Down;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Down);
                }
                Facing::Right => {
                    self.my_facing = Facing::Up;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Up);
                }
                Facing::Up => {
                    self.my_facing = Facing::Left;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Left);
                }
                Facing::Down => {
                    self.my_facing = Facing::Right;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Right);
                }
            },
        }
    }

    // Each cube face is a 50x50 grid. One way to do this
    // is to "teleport" the location to the correct place +
    // orientation whenever we step off a face.
    // The input is of this shape
    //   1 2
    //   3
    // 5 4
    // 6
    //
    // 1:
    //         6
    //       5 1 2
    //         3
    // When moving from 1 -> 2 and 3, the orientation is kept the same.
    // going off 1 on the left, results in coming into 5 from the left (with inverted indices)
    // going off 1 at the top results in coming into 6 from the left
    // 2:
    //         6
    //       1 2 4
    //         3
    // When moving from 2 -> 1, the orientation is kept the same.
    // going off 2 on the top, results in coming into 6 from the bottom
    // going off 2 at the right results in coming into 4 from the right (with inverted Y)
    // going off 2 at the bottom results in coming into 3 from the right
    // 3:
    //         1
    //       5 3 2
    //         4
    // When moving from 3 -> 1, 4, the orientation is kept the same.
    // going off 3 on the left, results in coming into 5 from the top
    // going off 3 at the right results in coming into 2 from the bottom
    // 4:
    //         3
    //       5 4 2
    //         6
    // When moving from 4 -> 3 and 5, the orientation is kept the same.
    // going off 4 on the right, results in coming into 2 from the right (with inverted Y indices)
    // going off 4 at the bottom results in coming into 6 from the right
    // 5:
    //         3
    //       1 5 4
    //         6
    // When moving from 5 -> 4, 6, the orientation is kept the same.
    // going off 5 on the left, results in coming into 1 from the left (with inverted Y)
    // going off 5 at the top results in coming into 3 from the left
    // 6:
    //         5
    //       1 6 4
    //         2
    // When moving from 6 -> 5, the orientation is kept the same.
    // going off 6 on the left, results in coming into 1 from the top
    // going off 6 at the bottom results in coming into 2 from the top
    // going off 6 at the right results in coming into 4 from the bottom
    const SIDE1_BOUNDS: ((usize, usize), (usize, usize)) = ((50, 100), (0, 50));
    const SIDE2_BOUNDS: ((usize, usize), (usize, usize)) = ((100, 150), (0, 50));
    const SIDE3_BOUNDS: ((usize, usize), (usize, usize)) = ((50, 100), (50, 100));
    const SIDE4_BOUNDS: ((usize, usize), (usize, usize)) = ((50, 100), (100, 150));
    const SIDE5_BOUNDS: ((usize, usize), (usize, usize)) = ((0, 50), (100, 150));
    const SIDE6_BOUNDS: ((usize, usize), (usize, usize)) = ((0, 50), (150, 200));

    fn is_in_bound(coord: (usize, usize), s: Side) -> bool {
        let bound = match s {
            Side::One => Self::SIDE1_BOUNDS,
            Side::Two => Self::SIDE2_BOUNDS,
            Side::Three => Self::SIDE3_BOUNDS,
            Side::Four => Self::SIDE4_BOUNDS,
            Side::Five => Self::SIDE5_BOUNDS,
            Side::Six => Self::SIDE6_BOUNDS,
        };

        coord.0 >= bound.0 .0
            && coord.0 < bound.0 .1
            && coord.1 >= bound.1 .0
            && coord.1 < bound.1 .1
    }

    fn get_side(&self) -> Side {
        if self.my_location.0 >= Self::SIDE1_BOUNDS.0 .0
            && self.my_location.0 < Self::SIDE1_BOUNDS.0 .1
            && self.my_location.1 >= Self::SIDE1_BOUNDS.1 .0
            && self.my_location.1 < Self::SIDE1_BOUNDS.1 .1
        {
            Side::One
        } else if self.my_location.0 >= Self::SIDE2_BOUNDS.0 .0
            && self.my_location.0 < Self::SIDE2_BOUNDS.0 .1
            && self.my_location.1 >= Self::SIDE2_BOUNDS.1 .0
            && self.my_location.1 < Self::SIDE2_BOUNDS.1 .1
        {
            Side::Two
        } else if self.my_location.0 >= Self::SIDE3_BOUNDS.0 .0
            && self.my_location.0 < Self::SIDE3_BOUNDS.0 .1
            && self.my_location.1 >= Self::SIDE3_BOUNDS.1 .0
            && self.my_location.1 < Self::SIDE3_BOUNDS.1 .1
        {
            Side::Three
        } else if self.my_location.0 >= Self::SIDE4_BOUNDS.0 .0
            && self.my_location.0 < Self::SIDE4_BOUNDS.0 .1
            && self.my_location.1 >= Self::SIDE4_BOUNDS.1 .0
            && self.my_location.1 < Self::SIDE4_BOUNDS.1 .1
        {
            Side::Four
        } else if self.my_location.0 >= Self::SIDE5_BOUNDS.0 .0
            && self.my_location.0 < Self::SIDE5_BOUNDS.0 .1
            && self.my_location.1 >= Self::SIDE5_BOUNDS.1 .0
            && self.my_location.1 < Self::SIDE5_BOUNDS.1 .1
        {
            Side::Five
        } else if self.my_location.0 >= Self::SIDE6_BOUNDS.0 .0
            && self.my_location.0 < Self::SIDE6_BOUNDS.0 .1
            && self.my_location.1 >= Self::SIDE6_BOUNDS.1 .0
            && self.my_location.1 < Self::SIDE6_BOUNDS.1 .1
        {
            Side::Six
        } else {
            panic!("unexpected location")
        }
    }

    // Peeks the next location based on the current position of self & whether the next
    // slot is free.
    fn next_location(&self) -> ((usize, usize), Facing, bool) {
        let current_side = self.get_side();
        let (next_loc, next_facing) = match self.my_facing {
            Facing::Left => {
                match current_side {
                    Side::Two | Side::Four => {
                        ((self.my_location.0 - 1, self.my_location.1), Facing::Left)
                    }
                    Side::One => {
                        if self.my_location.0 == 50 {
                            // We have stepped off 1 to the left, so go to 5 from the left, flipping Y
                            debug_assert!(Self::is_in_bound(
                                (0, 149 - self.my_location.1),
                                Side::Five
                            ));

                            ((0, 149 - self.my_location.1), Facing::Right)
                        } else {
                            ((self.my_location.0 - 1, self.my_location.1), Facing::Left)
                        }
                    }
                    Side::Three => {
                        if self.my_location.0 == 50 {
                            // Step off 3 on the left, go to 5 from the top
                            debug_assert!(Self::is_in_bound(
                                (self.my_location.1 - 50, 100),
                                Side::Five
                            ));

                            ((self.my_location.1 - 50, 100), Facing::Down)
                        } else {
                            ((self.my_location.0 - 1, self.my_location.1), Facing::Left)
                        }
                    }
                    Side::Five => {
                        if self.my_location.0 == 0 {
                            // Step off 5 on the left, go to 1 from the left with inverted Y
                            debug_assert!(Self::is_in_bound(
                                (50, 149 - self.my_location.1),
                                Side::One
                            ));

                            ((50, 149 - self.my_location.1), Facing::Right)
                        } else {
                            ((self.my_location.0 - 1, self.my_location.1), Facing::Left)
                        }
                    }
                    Side::Six => {
                        if self.my_location.0 == 0 {
                            // Step off 6 on the left, go to 1 from the top
                            debug_assert!(Self::is_in_bound(
                                (self.my_location.1 - 100, 0),
                                Side::One
                            ));

                            ((self.my_location.1 - 100, 0), Facing::Down)
                        } else {
                            ((self.my_location.0 - 1, self.my_location.1), Facing::Left)
                        }
                    }
                }
            }
            Facing::Right => match current_side {
                Side::One | Side::Five => {
                    ((self.my_location.0 + 1, self.my_location.1), Facing::Right)
                }
                Side::Two => {
                    if self.my_location.0 == 149 {
                        debug_assert!(Self::is_in_bound(
                            (99, 100 + (49 - self.my_location.1)),
                            Side::Four
                        ));
                        ((99, 100 + (49 - self.my_location.1)), Facing::Left)
                    } else {
                        ((self.my_location.0 + 1, self.my_location.1), Facing::Right)
                    }
                }
                Side::Three => {
                    if self.my_location.0 == 99 {
                        debug_assert!(Self::is_in_bound(
                            (100 + (self.my_location.1 - 50), 49),
                            Side::Two
                        ));
                        ((100 + (self.my_location.1 - 50), 49), Facing::Up)
                    } else {
                        ((self.my_location.0 + 1, self.my_location.1), Facing::Right)
                    }
                }
                Side::Four => {
                    if self.my_location.0 == 99 {
                        debug_assert!(Self::is_in_bound(
                            (149, 149 - self.my_location.1),
                            Side::Two
                        ));

                        ((149, 149 - self.my_location.1), Facing::Left)
                    } else {
                        ((self.my_location.0 + 1, self.my_location.1), Facing::Right)
                    }
                }
                Side::Six => {
                    if self.my_location.0 == 49 {
                        debug_assert!(Self::is_in_bound(
                            (50 + (self.my_location.1 - 150), 149),
                            Side::Four
                        ));
                        ((50 + (self.my_location.1 - 150), 149), Facing::Up)
                    } else {
                        ((self.my_location.0 + 1, self.my_location.1), Facing::Right)
                    }
                }
            },
            Facing::Up => match current_side {
                Side::Three | Side::Four | Side::Six => {
                    ((self.my_location.0, self.my_location.1 - 1), Facing::Up)
                }
                Side::One => {
                    if self.my_location.1 == 0 {
                        debug_assert!(Self::is_in_bound(
                            (0, 150 + (self.my_location.0 - 50)),
                            Side::Six
                        ));

                        ((0, 150 + (self.my_location.0 - 50)), Facing::Right)
                    } else {
                        ((self.my_location.0, self.my_location.1 - 1), Facing::Up)
                    }
                }
                Side::Two => {
                    if self.my_location.1 == 0 {
                        debug_assert!(Self::is_in_bound(
                            (self.my_location.0 - 100, 199),
                            Side::Six
                        ));

                        ((self.my_location.0 - 100, 199), Facing::Up)
                    } else {
                        ((self.my_location.0, self.my_location.1 - 1), Facing::Up)
                    }
                }
                Side::Five => {
                    if self.my_location.1 == 100 {
                        debug_assert!(Self::is_in_bound(
                            (50, 50 + (self.my_location.0)),
                            Side::Three
                        ));

                        ((50, 50 + (self.my_location.0)), Facing::Right)
                    } else {
                        ((self.my_location.0, self.my_location.1 - 1), Facing::Up)
                    }
                }
            },
            Facing::Down => match current_side {
                Side::One | Side::Three | Side::Five => {
                    ((self.my_location.0, self.my_location.1 + 1), Facing::Down)
                }
                Side::Two => {
                    if self.my_location.1 == 49 {
                        debug_assert!(Self::is_in_bound(
                            (99, 50 + (self.my_location.0 - 100)),
                            Side::Three
                        ));

                        ((99, 50 + (self.my_location.0 - 100)), Facing::Left)
                    } else {
                        ((self.my_location.0, self.my_location.1 + 1), Facing::Down)
                    }
                }
                Side::Four => {
                    if self.my_location.1 == 149 {
                        debug_assert!(Self::is_in_bound(
                            (49, 150 + (self.my_location.0 - 50)),
                            Side::Six
                        ));

                        ((49, 150 + (self.my_location.0 - 50)), Facing::Left)
                    } else {
                        ((self.my_location.0, self.my_location.1 + 1), Facing::Down)
                    }
                }
                Side::Six => {
                    if self.my_location.1 == 199 {
                        debug_assert!(Self::is_in_bound((100 + self.my_location.0, 0), Side::Two));

                        ((100 + self.my_location.0, 0), Facing::Down)
                    } else {
                        ((self.my_location.0, self.my_location.1 + 1), Facing::Down)
                    }
                }
            },
        };

        if self.map[next_loc.1][next_loc.0].is_free_space() {
            (next_loc, next_facing, true)
        } else {
            (next_loc, next_facing, false)
        }
    }

    fn do_instruction_cube(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Move(num_move) => {
                for _ in 0..num_move {
                    let (next_loc, next_facing, is_free) = self.next_location();
                    if !is_free {
                        break;
                    }
                    // Draw path over previous
                    self.map[self.my_location.1][self.my_location.0] = Square::Path(self.my_facing);
                    // Draw myself next
                    self.map[next_loc.1][next_loc.0] = Square::Me(next_facing);

                    // Update position
                    self.my_location = next_loc;
                    self.my_facing = next_facing;
                }
            }
            Instruction::RotateRight => match self.my_facing {
                Facing::Left => {
                    self.my_facing = Facing::Up;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Up);
                }
                Facing::Right => {
                    self.my_facing = Facing::Down;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Down);
                }
                Facing::Up => {
                    self.my_facing = Facing::Right;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Right);
                }
                Facing::Down => {
                    self.my_facing = Facing::Left;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Left);
                }
            },
            Instruction::RotateLeft => match self.my_facing {
                Facing::Left => {
                    self.my_facing = Facing::Down;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Down);
                }
                Facing::Right => {
                    self.my_facing = Facing::Up;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Up);
                }
                Facing::Up => {
                    self.my_facing = Facing::Left;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Left);
                }
                Facing::Down => {
                    self.my_facing = Facing::Right;
                    self.map[self.my_location.1][self.my_location.0] = Square::Me(Facing::Right);
                }
            },
        }
    }

    fn get_score(&self) -> u64 {
        let facing_score = match self.my_facing {
            Facing::Left => 2,
            Facing::Right => 0,
            Facing::Up => 3,
            Facing::Down => 1,
        };
        (self.my_location.1 + 1) as u64 * 1000 + (self.my_location.0 + 1) as u64 * 4 + facing_score
    }

    #[allow(unused)]
    fn print(&self) {
        for y in (0..self.map.len()) {
            eprint!("{:0>5} ", y);
            for x in (0..self.map[y].len()) {
                match self.map[y][x] {
                    Square::Me(direction) => match direction {
                        Facing::Left => eprint!("\x1B[0;31m<\x1b[0m"),
                        Facing::Right => eprint!("\x1B[0;31m>\x1b[0m"),
                        Facing::Up => eprint!("\x1B[0;31m^\x1b[0m"),
                        Facing::Down => eprint!("\x1B[0;31mv\x1b[0m"),
                    },
                    Square::Path(direction) => match direction {
                        Facing::Left => eprint!("<"),
                        Facing::Right => eprint!(">"),
                        Facing::Up => eprint!("^"),
                        Facing::Down => eprint!("v"),
                    },
                    Square::Open => eprint!("."),
                    Square::Wall => eprint!("#"),
                    Square::None => eprint!(" "),
                }
            }
            eprintln!();
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Me(Facing),
    Path(Facing),
    Open,
    Wall,
    None,
}

impl Square {
    fn is_free_space(&self) -> bool {
        match self {
            Square::None | Square::Me(_) => panic!("unexpected call {:?}", self),
            Square::Path(_) | Square::Open => true,
            Square::Wall => false,
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Move(usize),
    RotateRight,
    RotateLeft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Side {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}
