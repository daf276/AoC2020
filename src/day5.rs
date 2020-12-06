use itertools::Itertools;

pub fn read_input() -> Vec<u16> {
    return include_str!("../input/day5")
        .lines()
        .map(|line| parse_line(line))
        .collect();
}

fn parse_line(line: &str) -> u16 {
    return line
        .bytes()
        .map(|symbol| symbol == 'B' as u8 || symbol == 'R' as u8)
        .fold(0, |acc, bit| acc << 1 ^ bit as u16);
}

pub fn part1(input: &Vec<u16>) -> u16 {
    return *input.iter().max().unwrap();
}

pub fn part2(input: &Vec<u16>) -> u16 {
    let mut seat_ids = input.clone();
    seat_ids.sort_unstable();
    return seat_ids
        .iter()
        .tuple_windows()
        .find_map(|(a, &b)| (a + 1 != b).then_some(a + 1))
        .unwrap();
}
