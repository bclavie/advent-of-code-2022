use aoc2022::time_run2;

const INPUT: &str = include_str!("../inputs/10");

#[time_run2("10")]
fn main() {
    cathode_ray_tube(INPUT)
}

enum Instruction {
    Noop,
    Addx(i64),
}

impl Instruction {
    fn from_str(s: &str) -> Self {
        if s.starts_with("noop") {
            Self::Noop
        } else if s.starts_with("addx") {
            let (_, amount) = s.split_once(' ').unwrap();
            Self::Addx(amount.parse::<i64>().unwrap())
        } else {
            panic!("unexpected: {}", s)
        }
    }
}

const RECORDED_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

fn increment_and_maybe_record(register: i64, cycle: &mut usize, total: &mut i64) {
    *cycle += 1;
    if RECORDED_CYCLES.contains(cycle) {
        *total += register * *cycle as i64;
    }
}

fn increment_and_draw_screen(register: i64, cycle: &mut usize, screen: &mut [[char; 40]; 8]) {
    let row = (*cycle) / 40;
    let column = (*cycle) % 40;
    *cycle += 1;

    if (register - 1..=register + 1).contains(&(column as i64)) {
        screen[row][column] = '#'
    } else {
        screen[row][column] = '.'
    }
}

fn cathode_ray_tube(i: &str) -> (String, String) {
    let instructions: Vec<Instruction> = i.lines().map(Instruction::from_str).collect();
    let mut cycle1 = 0;
    let mut register1 = 1;
    let mut total = 0;

    let mut cycle2 = 0;
    let mut register2 = 1;
    let mut screen: [[char; 40]; 8] = [[' '; 40]; 8];

    for i in instructions {
        match i {
            Instruction::Noop => {
                increment_and_maybe_record(register1, &mut cycle1, &mut total);
                increment_and_draw_screen(register2, &mut cycle2, &mut screen);
            }
            Instruction::Addx(x) => {
                increment_and_maybe_record(register1, &mut cycle1, &mut total);
                increment_and_maybe_record(register1, &mut cycle1, &mut total);
                register1 += x;

                increment_and_draw_screen(register2, &mut cycle2, &mut screen);
                increment_and_draw_screen(register2, &mut cycle2, &mut screen);
                register2 += x;
            }
        }
    }

    let screen_str = screen
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .fold(String::default(), |acc, line| acc + &line + "\n");

    (total.to_string(), screen_str)
}
