use std::str;

#[derive(Clone)]
struct State {
    x: isize,
    cycle: usize,
}

enum Op {
    AddX(isize),
    Nop,
}

impl Op {
    fn execute(&self, state: &mut State) {
        match self {
            Self::AddX(operand) => {
                state.x += operand;
                state.cycle += 2;
            }
            Self::Nop => state.cycle += 1,
        }
    }
}

impl From<&str> for Op {
    fn from(op_str: &str) -> Self {
        let (op_name, rest) = op_str
            .split_once(' ')
            .map_or((op_str, None), |(n, r)| (n, Some(r)));

        match op_name {
            "addx" => Self::AddX(rest.unwrap().parse().unwrap()),
            "noop" => Self::Nop,
            _ => panic!("unknown opcode"),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let program = input
        .lines()
        .map(Op::from)
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let exec_trace = program
        .iter()
        .scan(State { x: 1, cycle: 0 }, |state, op| {
            op.execute(state);
            Some(state.clone())
        })
        .collect::<Vec<_>>();

    let x_for_cycle = |cycle| {
        exec_trace
            .iter()
            .map_while(|state| (state.cycle < cycle).then_some(state.x))
            .last()
            .unwrap_or(1)
    };

    let sum: isize = [20usize, 60, 100, 140, 180, 220]
        .iter()
        .copied()
        .map(|cycle| x_for_cycle(cycle) * cycle as isize)
        .sum();

    println!("p1: {:?}", sum);

    let mut screen = String::new();

    for cycle in 1..=240 {
        let x = x_for_cycle(cycle);
        let scan = (cycle % 40) as isize;

        if scan >= x && scan < x + 3 {
            screen += "#";
        } else {
            screen += ".";
        }
    }

    println!("p2: ");
    for line in screen.as_bytes().chunks(40).map(str::from_utf8) {
        println!("{:?}", line.unwrap());
    }
}
