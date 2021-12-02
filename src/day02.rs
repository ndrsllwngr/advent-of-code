use std::str::FromStr;

use itertools::Itertools;

pub fn part1(input: &String) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in input.lines() {
        match parse_line(&line.to_string()) {
            Action::Forward(v) => {
                horizontal_pos = horizontal_pos + v;
            }
            Action::Up(v) => {
                depth = depth - v;
            }
            Action::Down(v) => {
                depth = depth + v;
            }
            Action::Other => {}
        }
    }
    horizontal_pos * depth
}

enum Action {
    Forward(i32),
    Up(i32),
    Down(i32),
    Other,
}

fn parse_line(line: &String) -> Action {
    match &line {
        s if s.contains("forward") => Action::Forward(get_value(s)),
        s if s.contains("up") => Action::Up(get_value(s)),
        s if s.contains("down") => Action::Down(get_value(s)),
        _ => Action::Other,
    }
}

fn get_value(line: &String) -> i32 {
    let vec: Vec<&str> = line.split(' ').collect();
    <i32 as FromStr>::from_str(vec.last().unwrap()).unwrap()
}

pub fn part2(input: &String) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    for line in input.lines() {
        match parse_line(&line.to_string()) {
            Action::Forward(v) => {
                horizontal_pos = horizontal_pos + v;
                depth = depth + (aim * v);
            }
            Action::Up(v) => {
                aim = aim - v;
            }
            Action::Down(v) => {
                aim = aim + v;
            }
            Action::Other => {}
        }
    }
    horizontal_pos * depth
}