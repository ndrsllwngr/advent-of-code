use std::env;
use std::fmt::{Debug, format};
use std::fs;
use std::str::FromStr;

pub mod year2022;

pub fn read_input(year: u16, day: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let path = format!("input/{year}/day/day{day:02}.txt");
    let filename = cwd.join(path.clone());
    fs::read_to_string(filename).expect(&*format!("Error while reading file from {path}"))
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
