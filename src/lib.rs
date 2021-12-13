use std::env;
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day13;

pub fn read_input(day_num: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let filename = cwd.join("input").join(format!("day{:02}.txt", day_num));
    fs::read_to_string(filename).expect("Error while reading")
}

// utility functions

pub fn list_to_vec<T: FromStr>(input: &String) -> Vec<T>
where
    <T as FromStr>::Err: Debug,
{
    input
        .trim()
        .split(',')
        .map(str::parse::<T>)
        .map(Result::unwrap)
        .collect::<Vec<T>>()
}
