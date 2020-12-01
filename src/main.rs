use itertools::Itertools;
use std::time::{Duration, Instant};

fn main() {
    let input: Vec<u32> = include_str!("../day1")
        .lines()
        .map(|line| line.parse::<u32>().unwrap())
        .collect();

    let now = Instant::now();
    println!("Product: {}", part1(&input));
    println!("Elapsed µs: {}", now.elapsed().as_micros());

    let now = Instant::now();
    println!("Product: {}", part2(&input));
    println!("Elapsed µs: {}", now.elapsed().as_micros());
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
    return input
        .iter()
        .tuple_combinations()
        .filter(|(&a, &b, &c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c)
        .next()
        .unwrap();
}
