pub fn read_input() -> Vec<Vec<u128>> {
    include_str!("../input/day6")
        .replace("\r", "")
        .split("\n\n")
        .map(|grp| {
            grp.lines()
                .map(|person| person.bytes().fold(0, |acc, bit| acc ^ 1 << bit))
                .collect()
        })
        .collect()
}

pub fn part1(answers: &Vec<Vec<u128>>) -> u32 {
    return answers
        .iter()
        .map(|group| group.iter().fold(0, |acc, n| acc | n).count_ones())
        .sum();
}

pub fn part2(answers: &Vec<Vec<u128>>) -> u32 {
    return answers
        .iter()
        .map(|group| group.iter().fold(u128::MAX, |acc, n| acc & n).count_ones())
        .sum();
}
