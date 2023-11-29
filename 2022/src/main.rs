use std::env;
use std::io;
use aoc::read_input;

use aoc::*;


fn main() {
    let result_1: u64 = year2022::day01::part1(&read_input(2022, 01));
    let result_2: u64 = year2022::day01::part2(&read_input(2022, 01));
    println!("result_part_1={result_1}");
    println!("result_part_2={result_2}");
}