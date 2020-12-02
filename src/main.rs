mod day1;
mod day2;

fn main() {
    let day1_input = day1::read_input();
    println!("Day1 Part1: {}", day1::part1(&day1_input));
    println!("Day1 Part2: {}", day1::part2(&day1_input));

    let day2_input = day2::read_input();
    println!("Day2 Part1: {}", day2::part1(&day2_input));
    println!("Day2 Part2: {}", day2::part2(&day2_input));
}
