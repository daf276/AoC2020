#![feature(bool_to_option)]
#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let day1_input = day1::read_input();
    println!("Day1 Part1: {}", day1::part1(&day1_input));
    println!("Day1 Part2: {}", day1::part2(&day1_input));

    let day2_input = day2::read_input();
    println!("Day2 Part1: {}", day2::part1(&day2_input));
    println!("Day2 Part2: {}", day2::part2(&day2_input));

    let day3_input = day3::read_input();
    println!("Day3 Part1: {}", day3::part1(&day3_input));
    println!("Day3 Part2: {}", day3::part2(&day3_input));

    let day4_input = day4::read_input();
    println!("Day4 Part1: {}", day4::part1(&day4_input));
    println!("Day4 Part2: {}", day4::part2(&day4_input));

    let day5_input = day5::read_input();
    println!("Day5 Part1: {}", day5::part1(&day5_input));
    println!("Day5 Part2: {}", day5::part2(&day5_input));

    let day6_input = day6::read_input();
    println!("Day6 Part1: {}", day6::part1(&day6_input));
    println!("Day6 Part2: {}", day6::part2(&day6_input));

    let day7_input = day7::read_input();
    println!("Day7 Part1: {}", day7::part1(&day7_input));
    println!("Day7 Part2: {}", day7::part2(&day7_input));
}
