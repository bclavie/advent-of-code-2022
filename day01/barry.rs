fn elves(i: &str) -> String {
    let mut totals: Vec<u64> = i
        .split("\n\n")
        .map(|elf| {
            elf.split("\n")
                .map(|calorie| u64::from_str_radix(calorie, 10).unwrap())
                .fold(0, |acc, c| acc + c)
        })
        .collect();
    totals.sort_by(|a, b| b.cmp(a));

    let top_3: Vec<u64> = totals.into_iter().take(3).collect();
    let top_3_total = top_3.iter().fold(0, |acc, t| acc + t);

    top_3_total.to_string()
}
