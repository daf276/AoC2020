use itertools::Itertools;
use text_io::scan;

pub struct PasswordPolicy {
    character: char,
    min: usize,
    max: usize,
    pw: String,
}

impl From<&'_ str> for PasswordPolicy {
    fn from(s: &'_ str) -> Self {
        let character: char;
        let min: usize;
        let max: usize;
        let pw: String;
        scan!(s.bytes() => "{}-{} {}: {}\n", min, max, character, pw);
        PasswordPolicy {
            character,
            min,
            max,
            pw,
        }
    }
}

pub fn read_input() -> Vec<PasswordPolicy> {
    include_str!("../day2").lines().map_into().collect()
}

pub fn part1(input: &Vec<PasswordPolicy>) -> usize {
    return input
        .iter()
        .filter(|&policy| character_in_range(policy))
        .count();
}

pub fn part2(input: &Vec<PasswordPolicy>) -> usize {
    return input
        .iter()
        .filter(|&policy| characters_match(policy))
        .count();
}

fn character_in_range(policy: &PasswordPolicy) -> bool {
    let no_matches = policy.pw.chars().filter(|&c| c == policy.character).count();
    return no_matches >= policy.min && no_matches <= policy.max;
}

fn characters_match(policy: &PasswordPolicy) -> bool {
    let pw = policy.pw.as_bytes();
    let first = pw[policy.min - 1] == policy.character as u8;
    let second = pw[policy.max - 1] == policy.character as u8;
    return first ^ second == true;
}
