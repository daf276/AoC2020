use std::ops::{BitAnd, BitOr};

pub fn read_input() -> Vec<Vec<u128>> {
    return std::fs::read_to_string("input/day6")
        .unwrap()
        .replace("\r", "")
        .split("\n\n")
        .map(|grp| {
            grp.lines()
                .map(|person| person.bytes().fold(0, |acc, bit| acc ^ 1 << bit))
                .collect()
        })
        .collect();
}

pub fn part1(answers: &Vec<Vec<u128>>) -> u32 {
    return answers
        .iter()
        .map(|group| group.iter().fold(0, u128::bitor).count_ones())
        .sum();
}

pub fn part2(answers: &Vec<Vec<u128>>) -> u32 {
    return answers
        .iter()
        .map(|group| group.iter().fold(u128::MAX, u128::bitand).count_ones())
        .sum();
}
