use itertools::iproduct;
use itertools::Itertools;

fn main() {
    let input: Vec<u32> = include_str!("../day1")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    println!("Product Part 1: {}", part1(&input));
    println!("Product Part 2: {}", part2(&input));
}

fn part1(input: &Vec<u32>) -> u32 {
    return input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .next()
        .unwrap();
}

fn part2(input: &Vec<u32>) -> u32 {
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
