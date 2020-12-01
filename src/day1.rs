use itertools::iproduct;
use itertools::Itertools;

pub fn read_input() -> Vec<u32> {
    return include_str!("../day1")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();
}

pub fn part1(input: &Vec<u32>) -> u32 {
    return input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .next()
        .unwrap();
}

pub fn part2(input: &Vec<u32>) -> u32 {
    let sum = input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b < 2020)
        .collect::<Vec<(&u32, &u32)>>();

    for ((a, b), c) in iproduct!(sum, input) {
        if a + b + c == 2020 {
            return a * b * c;
        }
    }
    return 0;
}
