use std::{env, fs};
use std::fmt::Debug;
use std::io::BufRead;
use std::str::FromStr;

pub fn read_input(day_number: u8) -> String {
    let cwd = env::current_dir().unwrap();
    let path = format!("inputs/{day_number:02}.txt");
    let filename = cwd.join(path.clone());
    let input = fs::read_to_string(filename).expect(&*format!("Error while reading file from {path}"));
    input
}

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