#![feature(test)]
extern crate test;

mod day1;
mod day2;

#[bench]
fn day1_part1(b: &mut test::Bencher) {
    let input = day1::read_input();
    b.iter(|| test::black_box(day1::part1(&input)))
}

#[bench]
fn day1_part2(b: &mut test::Bencher) {
    let input = day1::read_input();
    b.iter(|| test::black_box(day1::part2(&input)))
}

#[bench]
fn day2_part1(b: &mut test::Bencher) {
    let input = day2::read_input();
    b.iter(|| test::black_box(day2::part1(&input)))
}

#[bench]
fn day2_part2(b: &mut test::Bencher) {
    let input = day2::read_input();
    b.iter(|| test::black_box(day2::part2(&input)))
}
