fn main() {
    //  seperate our large string into a newline seperated list
    let input = include_str!("../input.txt")
        .split(|c| c == '\n')
        .collect::<Vec<_>>();

    // take our input list and further split it into groups by
    // splitting the empty line. then transform each sublist into
    // a integer by converting all elements to an integer and summing
    let mut elf_calories = input
        .split(|&w| w.is_empty())
        .map(|l| {
            l.iter()
                .map(|&s| s.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    // we can safely unstable sort these total calories as we're not
    // interested in which elf is carrying which amount. sort in
    // decesending order.
    elf_calories.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());

    // part 1
    println!("p1: {}", elf_calories[0]);

    // part 2
    println!("p2: {}", elf_calories[0] + elf_calories[1] + elf_calories[2]);
}
