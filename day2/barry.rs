use aoc2022::time_run;

const INPUT: &str = include_str!("../inputs/02");

#[time_run("02")]
fn main() {
    rps2(INPUT)
}

#[derive(Copy, Clone)]
enum OpponentMove {
    Rock,
    Paper,
    Scissor,
}

impl From<char> for OpponentMove {
    fn from(c: char) -> Self {
        match c {
            'A' => Self::Rock,
            'B' => Self::Paper,
            'C' => Self::Scissor,
            _ => panic!("unexpected opponent move"),
        }
    }
}

#[derive(Copy, Clone)]
enum MyMove {
    Rock,
    Paper,
    Scissor,
}

impl MyMove {
    fn duel(&self, other: OpponentMove) -> u64 {
        match (self, other) {
            (MyMove::Rock, OpponentMove::Rock) => 1 + 3,
            (MyMove::Rock, OpponentMove::Paper) => 1 + 0,
            (MyMove::Rock, OpponentMove::Scissor) => 1 + 6,
            (MyMove::Paper, OpponentMove::Rock) => 2 + 6,
            (MyMove::Paper, OpponentMove::Paper) => 2 + 3,
            (MyMove::Paper, OpponentMove::Scissor) => 2 + 0,
            (MyMove::Scissor, OpponentMove::Rock) => 3 + 0,
            (MyMove::Scissor, OpponentMove::Paper) => 3 + 6,
            (MyMove::Scissor, OpponentMove::Scissor) => 3 + 3,
        }
    }
}

impl From<char> for MyMove {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Rock,
            'Y' => Self::Paper,
            'Z' => Self::Scissor,
            _ => panic!("unexpected opponent move"),
        }
    }
}

#[derive(Copy, Clone)]
enum RequiredOutcome {
    Lose,
    Draw,
    Win,
}

impl RequiredOutcome {
    fn score_for_required_move(&self, other: OpponentMove) -> u64 {
        match (self, other) {
            (RequiredOutcome::Lose, OpponentMove::Rock) => 0 + 3,
            (RequiredOutcome::Lose, OpponentMove::Paper) => 0 + 1,
            (RequiredOutcome::Lose, OpponentMove::Scissor) => 0 + 2,
            (RequiredOutcome::Draw, OpponentMove::Rock) => 3 + 1,
            (RequiredOutcome::Draw, OpponentMove::Paper) => 3 + 2,
            (RequiredOutcome::Draw, OpponentMove::Scissor) => 3 + 3,
            (RequiredOutcome::Win, OpponentMove::Rock) => 6 + 2,
            (RequiredOutcome::Win, OpponentMove::Paper) => 6 + 3,
            (RequiredOutcome::Win, OpponentMove::Scissor) => 6 + 1,
        }
    }
}

impl From<char> for RequiredOutcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Self::Lose,
            'Y' => Self::Draw,
            'Z' => Self::Win,
            _ => panic!("unexpected opponent move"),
        }
    }
}

fn _rps1(input: &str) -> String {
    let moves: Vec<(OpponentMove, MyMove)> = input
        .split("\n")
        .map(|s| {
            let chars: Vec<char> = s.chars().collect();
            (OpponentMove::from(chars[0]), MyMove::from(chars[2]))
        })
        .collect();

    let total = moves
        .iter()
        .fold(0, |acc, moves| acc + moves.1.duel(moves.0));

    total.to_string()
}

fn rps2(input: &str) -> String {
    let moves: Vec<u64> = input
        .split("\n")
        .map(|s| {
            let chars: Vec<char> = s.chars().collect();
            let required_outcome = RequiredOutcome::from(chars[2]);
            required_outcome.score_for_required_move(OpponentMove::from(chars[0]))
        })
        .collect();

    let total = moves.iter().fold(0, |acc, t| acc + t);

    total.to_string()
}
