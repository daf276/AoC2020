#![allow(dead_code)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;

#[bench]
fn day1_input(b: &mut test::Bencher) {
    b.iter(|| test::black_box(day1::read_input()))
}

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
fn day2_input(b: &mut test::Bencher) {
    b.iter(|| test::black_box(day2::read_input()))
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

#[bench]
fn day3_input(b: &mut test::Bencher) {
    b.iter(|| test::black_box(day3::read_input()))
}

#[bench]
fn day3_part1(b: &mut test::Bencher) {
    let input = day3::read_input();
    b.iter(|| test::black_box(day3::part1(&input)))
}

#[bench]
fn day3_part2(b: &mut test::Bencher) {
    let input = day3::read_input();
    b.iter(|| test::black_box(day3::part2(&input)))
}

#[bench]
fn day4_input(b: &mut test::Bencher) {
    b.iter(|| test::black_box(day4::read_input()))
}

#[bench]
fn day4_part1(b: &mut test::Bencher) {
    let input = day4::read_input();
    b.iter(|| test::black_box(day4::part1(&input)))
}

#[bench]
fn day4_part2(b: &mut test::Bencher) {
    let input = day4::read_input();
    b.iter(|| test::black_box(day4::part2(&input)))
}
