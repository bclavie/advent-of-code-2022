use aoc2022::time_run;

const INPUT: &str = include_str!("../inputs/04");

#[time_run("04")]
fn main() {
    assignment_pairings(INPUT)
}

fn assignment_pairings(input: &str) -> String {
    let mut total = 0;
    for pairings in input.split("\n") {
        let (elf1, elf2) = pairings.split_once(",").unwrap();
        let (elf1_lower, elf1_upper) = elf1.split_once("-").map(pair_to_u64).unwrap();
        let (elf2_lower, elf2_upper) = elf2.split_once("-").map(pair_to_u64).unwrap();

        // Part 1
        // if (elf1_lower >= elf2_lower && elf1_upper <= elf2_upper)
        //     || (elf2_lower >= elf1_lower && elf2_upper <= elf1_upper)
        // {
        //     total += 1;
        // };

        // Part 2
        for section in elf1_lower..=elf1_upper {
            if (elf2_lower..=elf2_upper).contains(&section) {
                total += 1;
                break;
            }
        }
    }

    total.to_string()
}

fn pair_to_u64(s: (&str, &str)) -> (u64, u64) {
    (
        u64::from_str_radix(s.0, 10).unwrap(),
        u64::from_str_radix(s.1, 10).unwrap(),
    )
}
