use rayon::prelude::*;
use std::collections::HashSet;

pub fn read_input() -> Vec<Vec<HashSet<char>>> {
    include_str!("../input/day6")
        .replace("\r", "")
        .split("\n\n")
        .map(|grp| grp.lines().map(|person| person.chars().collect()).collect())
        .collect()
}

pub fn part1(answers: &Vec<Vec<HashSet<char>>>) -> usize {
    return answers.par_iter().map(|grp| union(grp).len()).sum();
}

pub fn part2(answers: &[Vec<HashSet<char>>]) -> usize {
    return answers.par_iter().map(|grp| intersection(grp).len()).sum();
}

fn union(sets: &Vec<HashSet<char>>) -> HashSet<char> {
    let mut result = HashSet::new();
    for set in sets.iter() {
        result = result.union(&set).map(|&c| c).collect();
    }
    return result;
}

fn intersection(sets: &Vec<HashSet<char>>) -> HashSet<char> {
    let mut result = sets[0].clone();
    for set in sets.iter() {
        result = result.intersection(&set).map(|&c| c).collect();
    }
    return result;
}
