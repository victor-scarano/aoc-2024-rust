#[aoc_runner_derive::aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
    let mut lhs = Vec::with_capacity(1000);
    let mut rhs = Vec::with_capacity(1000);
    for line in input.split('\n') {
        let mut entries = line.split_whitespace().map(|val| val.parse::<u32>().unwrap());
        lhs.push(entries.next().unwrap());
        rhs.push(entries.next().unwrap());
    };
    lhs.sort_unstable();
    rhs.sort_unstable();
    lhs.into_iter().zip(rhs.into_iter()).map(|(l, r)| l.abs_diff(r)).sum::<u32>()
}

#[aoc_runner_derive::aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    let mut lhs = Vec::with_capacity(1000);
    let mut rhs = Vec::with_capacity(1000);
    for line in input.split('\n') {
        let mut entries = line.split_whitespace().map(|val| val.parse::<u32>().unwrap());
        lhs.push(entries.next().unwrap());
        rhs.push(entries.next().unwrap());
    };

    use std::collections::HashMap;
    let mut occurences = HashMap::<_, u32>::with_capacity(1000);
    for val in rhs.iter() {
        *occurences.entry(*val).or_default() += 1;
    }

    lhs.iter().map(|val| *val * occurences.get(val).cloned().unwrap_or_default()).sum()
}
