use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

lazy_static! {
    static ref KEYS: Vec<&'static str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    static ref EYE_COLOR: Vec<&'static str> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"#[0-f]{6}").unwrap();
}

pub fn read_input() -> Vec<String> {
    let input = include_str!("../input/day4").lines().collect_vec();

    let mut vec: Vec<String> = Vec::new();

    let mut string = String::from("");

    for i in 0..input.len() - 1 {
        if input[i] != "" {
            string += input[i];
            if input[i + 1] != "" {
                string += " ";
            }
        } else {
            vec.push(string.clone());
            string = String::from("");
        }
    }

    return vec;
}

pub fn part1(input: &Vec<String>) -> usize {
    return filter_missing_key_passports(input).len();
}

pub fn part2(input: &Vec<String>) -> usize {
    let passports_with_correct_keys = filter_missing_key_passports(input);

    return passports_with_correct_keys
        .par_iter()
        .filter(|&passport| is_valid(passport))
        .count();
}

fn filter_missing_key_passports(input: &Vec<String>) -> Vec<&String> {
    return input
        .par_iter()
        .filter(|&line| KEYS.iter().all(|&key| line.contains(key)))
        .collect();
}

fn is_valid(passport: &str) -> bool {
    return passport
        .split(" ")
        .map(|kv_pair| separate_key_value(kv_pair))
        .all(|(key, value)| has_valid_value(key, value));
}

fn separate_key_value(pair: &str) -> (&str, &str) {
    return pair.split(":").collect_tuple().unwrap();
}

fn has_valid_value(key: &str, value: &str) -> bool {
    return match key {
        "byr" => is_between_years(value, 1920, 2002),
        "iyr" => is_between_years(value, 2010, 2020),
        "eyr" => is_between_years(value, 2020, 2030),
        "hgt" => is_valid_height(value),
        "hcl" => HAIR_COLOR_RE.is_match(value),
        "ecl" => is_valid_eye_color(value),
        "pid" => is_valid_passport_id(value),
        "cid" => true,
        _ => unreachable!(),
    };
}

fn is_between_years(input: &str, start: usize, end: usize) -> bool {
    return match input.parse::<usize>() {
        Ok(n) => n >= start && n <= end,
        Err(_) => false,
    };
}

fn is_valid_height(input: &str) -> bool {
    return if input.contains("in") {
        let (height, _) = input.split_at(2);
        match height.parse::<u32>() {
            Ok(n) => n >= 59 && n <= 76,
            Err(_) => false,
        }
    } else if input.contains("cm") {
        let (height, _) = input.split_at(3);
        match height.parse::<u32>() {
            Ok(n) => n >= 150 && n <= 193,
            Err(_) => false,
        }
    } else {
        false
    };
}

fn is_valid_eye_color(input: &str) -> bool {
    return EYE_COLOR.iter().any(|&color| color == input);
}

fn is_valid_passport_id(input: &str) -> bool {
    return match input.parse::<u32>() {
        Ok(_) => input.len() == 9,
        Err(_) => false,
    };
}
