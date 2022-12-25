use std::{fmt::Display, ops::Add};

use aoc2022::time_run2;
use itertools::Itertools;

const INPUT: &str = include_str!("../inputs/25");

#[time_run2("25")]
fn main() {
    full_of_hot_air(INPUT)
}

fn full_of_hot_air(i: &str) -> (String, String) {
    let numbers = i.lines().map(SnafuNumber::from_str).collect_vec();

    let part1: SnafuNumber = numbers
        .iter()
        .fold(SnafuNumber::from_str("0"), |acc, x| acc + x.clone());

    (part1.to_string(), "".to_string())
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SnafuNumber {
    digits: Vec<SnafuDigit>,
}

impl Display for SnafuNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in self.digits.iter().rev() {
            f.write_fmt(format_args!("{}", d))?
        }
        Ok(())
    }
}

impl SnafuNumber {
    fn from_str(i: &str) -> Self {
        let mut digits = vec![];
        for c in i.chars().rev() {
            match c {
                '2' => digits.push(SnafuDigit::Two),
                '1' => digits.push(SnafuDigit::One),
                '0' => digits.push(SnafuDigit::Zero),
                '-' => digits.push(SnafuDigit::Minus),
                '=' => digits.push(SnafuDigit::DoubleMinus),
                _ => panic!("unexpected char"),
            }
        }

        Self { digits }
    }

    fn take_one_at(&mut self, index: usize) {
        let mut end_index = index;
        while self.digits[end_index] == SnafuDigit::DoubleMinus {
            end_index += 1
        }

        if end_index > self.digits.len() - 1 {
            panic!("can't take 1")
        }

        for i in index..=end_index {
            self.digits[i] = self.digits[i].must_take_one();
        }
    }

    fn add_one_at(&mut self, index: usize) {
        let mut end_index = index;
        while self.digits[end_index] == SnafuDigit::Two {
            end_index += 1
        }

        if end_index > self.digits.len() - 1 {
            self.digits.push(SnafuDigit::Zero)
        }

        for i in index..=end_index {
            self.digits[i] = self.digits[i].must_add_one()
        }
    }
}

impl Add for SnafuNumber {
    type Output = SnafuNumber;

    // Only works for positive numbers.
    fn add(self, rhs: Self) -> Self::Output {
        let (biggest, smallest) = if self.digits.len() > rhs.digits.len() {
            (self, rhs)
        } else {
            (rhs, self)
        };

        let mut result = SnafuNumber {
            digits: Vec::<SnafuDigit>::new(),
        };
        for _ in 0..=biggest.digits.len() {
            result.digits.push(SnafuDigit::Zero);
        }

        for index in 0..biggest.digits.len() {
            let (lhs_digit, rhs_digit) = match (biggest.digits[index], smallest.digits.get(index)) {
                (lhs_digit, None) => (lhs_digit, SnafuDigit::Zero),
                (lhs_digit, Some(rhs_digit)) => (lhs_digit, *rhs_digit),
            };

            match (lhs_digit, rhs_digit) {
                (SnafuDigit::Two, SnafuDigit::Two) => {
                    for _ in 0..4 {
                        result.add_one_at(index)
                    }
                }
                (SnafuDigit::Two, SnafuDigit::One) | (SnafuDigit::One, SnafuDigit::Two) => {
                    for _ in 0..3 {
                        result.add_one_at(index)
                    }
                }
                (SnafuDigit::Two, SnafuDigit::Zero)
                | (SnafuDigit::One, SnafuDigit::One)
                | (SnafuDigit::Zero, SnafuDigit::Two) => {
                    for _ in 0..2 {
                        result.add_one_at(index)
                    }
                }
                (SnafuDigit::Two, SnafuDigit::Minus)
                | (SnafuDigit::Zero, SnafuDigit::One)
                | (SnafuDigit::One, SnafuDigit::Zero)
                | (SnafuDigit::Minus, SnafuDigit::Two) => result.add_one_at(index),

                (SnafuDigit::Two, SnafuDigit::DoubleMinus)
                | (SnafuDigit::Minus, SnafuDigit::One)
                | (SnafuDigit::DoubleMinus, SnafuDigit::Two)
                | (SnafuDigit::Zero, SnafuDigit::Zero)
                | (SnafuDigit::One, SnafuDigit::Minus) => {
                    // No change to the result digit here
                }

                (SnafuDigit::Minus, SnafuDigit::Zero)
                | (SnafuDigit::One, SnafuDigit::DoubleMinus)
                | (SnafuDigit::DoubleMinus, SnafuDigit::One)
                | (SnafuDigit::Zero, SnafuDigit::Minus) => result.take_one_at(index),

                (SnafuDigit::Zero, SnafuDigit::DoubleMinus)
                | (SnafuDigit::DoubleMinus, SnafuDigit::Zero)
                | (SnafuDigit::Minus, SnafuDigit::Minus) => {
                    for _ in 0..2 {
                        result.take_one_at(index)
                    }
                }
                (SnafuDigit::Minus, SnafuDigit::DoubleMinus)
                | (SnafuDigit::DoubleMinus, SnafuDigit::Minus) => {
                    for _ in 0..3 {
                        result.take_one_at(index)
                    }
                }
                (SnafuDigit::DoubleMinus, SnafuDigit::DoubleMinus) => {
                    for _ in 0..4 {
                        result.take_one_at(index)
                    }
                }
            }
        }

        if result.digits.last() == Some(&SnafuDigit::Zero) {
            result.digits.pop();
        }

        result
    }
}

#[cfg(test)]
mod test {
    use crate::SnafuNumber;

    #[test]
    fn test_add_zero() {
        let a = SnafuNumber::from_str("2-=12-=");
        let b = SnafuNumber::from_str("0");

        let result = a + b;
        assert_eq!(result, SnafuNumber::from_str("2-=12-="))
    }

    #[test]
    fn test_add_simple() {
        let a = SnafuNumber::from_str("11");
        let b = SnafuNumber::from_str("1-");

        let result = a + b;
        assert_eq!(result, SnafuNumber::from_str("20"))
    }

    #[test]
    fn test_add_1() {
        let a = SnafuNumber::from_str("11");
        let b = SnafuNumber::from_str("1-");

        let result = a + b;
        assert_eq!(result, SnafuNumber::from_str("20"))
    }

    #[test]
    fn example() {
        let i = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122"#;

        let numbers: Vec<SnafuNumber> = i.lines().map(SnafuNumber::from_str).collect();

        let answer: SnafuNumber = numbers
            .iter()
            .fold(SnafuNumber::from_str("0"), |acc, x| acc + x.clone());

        assert_eq!(answer, SnafuNumber::from_str("2=-1=0"))
    }

    #[test]
    fn test_add_borrowing() {
        let a = SnafuNumber::from_str("1=");
        let b = SnafuNumber::from_str("1=");

        let result = a + b;
        assert_eq!(result, SnafuNumber::from_str("11"))
    }

    #[test]
    fn test_add_borrowing_multiple() {
        let a = SnafuNumber::from_str("1===");
        let b = SnafuNumber::from_str("1=");

        let result = a + b;
        assert_eq!(result, SnafuNumber::from_str("1==1"))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SnafuDigit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl SnafuDigit {
    fn must_take_one(self) -> SnafuDigit {
        match self {
            SnafuDigit::Two => SnafuDigit::One,
            SnafuDigit::One => SnafuDigit::Zero,
            SnafuDigit::Zero => SnafuDigit::Minus,
            SnafuDigit::Minus => SnafuDigit::DoubleMinus,
            SnafuDigit::DoubleMinus => SnafuDigit::Two,
        }
    }

    fn must_add_one(self) -> SnafuDigit {
        match self {
            SnafuDigit::Two => SnafuDigit::DoubleMinus,
            SnafuDigit::One => SnafuDigit::Two,
            SnafuDigit::Zero => SnafuDigit::One,
            SnafuDigit::Minus => SnafuDigit::Zero,
            SnafuDigit::DoubleMinus => SnafuDigit::Minus,
        }
    }
}

impl Display for SnafuDigit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnafuDigit::Two => f.write_fmt(format_args!("2")),
            SnafuDigit::One => f.write_fmt(format_args!("1")),
            SnafuDigit::Zero => f.write_fmt(format_args!("0")),
            SnafuDigit::Minus => f.write_fmt(format_args!("-")),
            SnafuDigit::DoubleMinus => f.write_fmt(format_args!("=")),
        }
    }
}
