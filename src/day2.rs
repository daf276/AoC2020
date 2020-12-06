use rayon::prelude::*;
use text_io::scan;

pub struct PasswordPolicy {
    character: u8,
    min: usize,
    max: usize,
    pw: String,
}

impl From<&'_ str> for PasswordPolicy {
    fn from(s: &'_ str) -> Self {
        let c: char;
        let min: usize;
        let max: usize;
        let pw: String;
        scan!(s.bytes() => "{}-{} {}: {}", min, max, c, pw);
        let character = c as u8;
        PasswordPolicy {
            character,
            min,
            max,
            pw,
        }
    }
}

pub fn read_input() -> Vec<PasswordPolicy> {
    include_str!("../input/day2")
        .par_lines()
        .map(|line| PasswordPolicy::from(line))
        .collect()
}

pub fn part1(input: &Vec<PasswordPolicy>) -> usize {
    return input
        .iter()
        .fold(0, |acc, policy| acc + character_in_range(policy) as usize);
}

pub fn part2(input: &Vec<PasswordPolicy>) -> usize {
    return input
        .iter()
        .fold(0, |acc, policy| acc + characters_match(policy) as usize);
}

fn character_in_range(policy: &PasswordPolicy) -> bool {
    let no_matches = policy
        .pw
        .bytes()
        .fold(0, |acc, c| acc + (c == policy.character) as usize);
    return no_matches >= policy.min && no_matches <= policy.max;
}

fn characters_match(policy: &PasswordPolicy) -> bool {
    let pw = policy.pw.as_bytes();
    let first = pw[policy.min - 1] == policy.character;
    let second = pw[policy.max - 1] == policy.character;
    return first ^ second == true;
}
