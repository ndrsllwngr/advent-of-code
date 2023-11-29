use std::str::FromStr;

pub fn part1(input: &String) -> i32 {
    let mut horizontal_pos = 0;
    let mut depth = 0;
    for line in input.lines() {
        match parse_line(&line.to_string()) {
            Action::Forward(v) => {
                horizontal_pos += v;
            }
            Action::Up(v) => {
                depth -= v;
            }
            Action::Down(v) => {
                depth += v;
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

fn parse_line(line: &str) -> Action {
    match &line {
        s if s.contains("forward") => Action::Forward(get_value(s)),
        s if s.contains("up") => Action::Up(get_value(s)),
        s if s.contains("down") => Action::Down(get_value(s)),
        _ => Action::Other,
    }
}

fn get_value(line: &str) -> i32 {
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
                horizontal_pos += v;
                depth += aim * v;
            }
            Action::Up(v) => {
                aim -= v;
            }
            Action::Down(v) => {
                aim += v;
            }
            Action::Other => {}
        }
    }
    horizontal_pos * depth
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    use crate::year2021::YEAR;

    const DAY: u8 = 2;
    const VALID_ANSWER_PART_1: i32 = 1813801;
    const VALID_ANSWER_PART_2: i32 = 1960569556;

    #[test]
    fn validate_part_1() -> Result<(), String> {
        assert_eq!(super::part1(&read_input(YEAR, DAY)), VALID_ANSWER_PART_1);
        Ok(())
    }

    #[test]
    fn validate_part_2() -> Result<(), String> {
        assert_eq!(super::part2(&read_input(YEAR, DAY)), VALID_ANSWER_PART_2);
        Ok(())
    }
}
