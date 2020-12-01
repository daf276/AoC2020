mod day1;

fn main() {
    let input = day1::read_input();
    println!("Day1 Part1: {}", day1::part1(&input));
    println!("Day1 Part2: {}", day1::part2(&input));
}
