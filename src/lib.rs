#![feature(test)]

extern crate test;
use itertools::iproduct;
use itertools::Itertools;
use test::Bencher;

mod day1;

#[bench]
fn day1_part1(b: &mut Bencher) {
    let input = day1::read_input();
    b.iter(|| day1::part1(&input))
}

#[bench]
fn day1_part2(b: &mut Bencher) {
    let input = day1::read_input();
    b.iter(|| day1::part2(&input))
}
