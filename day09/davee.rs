use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Move {
    Up,
    Left,
    Right,
    Down,
}

fn to_move_list(op: &str) -> Vec<Move> {
    let (op, amount) = op
        .split_once(' ')
        .map(|(op, amount)| (op, amount.parse::<usize>().unwrap()))
        .unwrap();

    let op = match op {
        "U" => Move::Up,
        "L" => Move::Left,
        "R" => Move::Right,
        "D" => Move::Down,
        _ => panic!("unknown op"),
    };

    vec![op; amount]
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

fn update_tail_position(head: Position, tail: Position) -> Position {
    let x_distance = head.x - tail.x;
    let y_distance = head.y - tail.y;

    match (x_distance.abs() > 1, y_distance.abs() > 1) {
        (true, true) => Position {
            x: head.x - x_distance.signum(),
            y: head.y - y_distance.signum(),
        },

        (true, false) => Position {
            x: head.x - x_distance.signum(),
            y: head.y,
        },

        (false, true) => Position {
            x: head.x,
            y: head.y - y_distance.signum(),
        },

        (false, false) => tail,
    }
}

struct Rope {
    knots: Vec<Position>,
}

impl Rope {
    pub fn with_count(count: usize) -> Self {
        Self {
            knots: vec![Position::default(); count],
        }
    }

    pub fn move_head(&mut self, move_op: Move) {
        match move_op {
            Move::Up => self.knots[0].y += 1,
            Move::Left => self.knots[0].x -= 1,
            Move::Right => self.knots[0].x += 1,
            Move::Down => self.knots[0].y -= 1,
        }

        for index in 1..self.knots.len() {
            self.knots[index] = update_tail_position(self.knots[index - 1], self.knots[index]);
        }
    }
}

fn part1(moves: &[Move]) {
    let mut rope = Rope::with_count(2);
    let mut tail_visited_coords = HashSet::new();

    for move_op in moves.iter().copied() {
        rope.move_head(move_op);
        tail_visited_coords.insert(rope.knots.last().cloned().unwrap());
    }

    println!("p1: {}", tail_visited_coords.len());
}

fn part2(moves: &[Move]) {
    let mut rope = Rope::with_count(10);
    let mut tail_visited_coords = HashSet::new();

    for move_op in moves.iter().copied() {
        rope.move_head(move_op);
        tail_visited_coords.insert(rope.knots.last().cloned().unwrap());
    }

    println!("p2: {}", tail_visited_coords.len());
}

fn main() {
    let input = include_str!("../input.txt");

    let moves = input
        .split('\n')
        .map(to_move_list)
        .collect::<Vec<_>>()
        .concat()
        .into_boxed_slice();

    part1(&moves);
    part2(&moves);
}
