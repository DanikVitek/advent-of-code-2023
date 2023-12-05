use day_1::*;

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(INPUT));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(INPUT));
}
