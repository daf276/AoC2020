#![allow(dead_code)]
#![feature(test)]
#![feature(bool_to_option)]
extern crate test;

#[macro_use]
extern crate lazy_static;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

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

#[bench]
fn day5_input(b: &mut test::Bencher) {
    b.iter(|| test::black_box(day5::read_input()))
}

#[bench]
fn day5_part1(b: &mut test::Bencher) {
    let input = day5::read_input();
    b.iter(|| test::black_box(day5::part1(&input)))
}

#[bench]
fn day5_part2(b: &mut test::Bencher) {
    let input = day5::read_input();
    b.iter(|| test::black_box(day5::part2(&input)))
}

#[bench]
fn day6_input(b: &mut test::Bencher) {
    b.iter(|| test::black_box(day6::read_input()))
}

#[bench]
fn day6_part1(b: &mut test::Bencher) {
    let input = day6::read_input();
    b.iter(|| test::black_box(day6::part1(&input)))
}

#[bench]
fn day6_part2(b: &mut test::Bencher) {
    let input = day6::read_input();
    b.iter(|| test::black_box(day6::part2(&input)))
}
