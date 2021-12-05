use std::str::FromStr;

pub fn part1(input: &String) -> i32 {
    let line_len = &input.lines().last().unwrap().len();
    let mut sum: Vec<i32> = vec![0; *line_len];

    let count = &input.lines().count();

    for (_i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().into_iter().enumerate() {
            let prev_value = sum[j];
            sum[j] = prev_value + <i32 as FromStr>::from_str(&char.to_string()).unwrap();
        }
    }

    let big: Vec<i32> = sum
        .clone()
        .into_iter()
        .map(|v| {
            if v >= *(count) as i32 - v {
                return 1;
            }
            0
        })
        .collect();
    let small: Vec<i32> = sum
        .clone()
        .into_iter()
        .map(|v| {
            if v >= (count / 2) as i32 {
                return 0;
            }
            1
        })
        .collect();
    let big_str: String = big.clone().into_iter().map(|i| i.to_string()).collect();
    let small_str: String = small.clone().into_iter().map(|i| i.to_string()).collect();

    i32::from_str_radix(&*big_str, 2).unwrap() * i32::from_str_radix(&*small_str, 2).unwrap()
}

pub fn part2(input: &String) -> u32 {
    let lines = input
        .lines()
        // could use bit twiddles and leave each line as a u16, but I'm lazy
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect::<Vec<Vec<u8>>>();

    let len = lines.get(0).unwrap().len();

    if let Some(ox) = filter(lines.clone(), len, true) {
        let co2 = filter(lines, len, false).unwrap();
        return ox * co2;
    }
    20
}

fn filter(mut lines: Vec<Vec<u8>>, len: usize, most_common: bool) -> Option<u32> {
    for col in 0..len {
        let zeroes = lines.iter().filter(|line| line[col] == 0).count();
        let ones = lines.len() - zeroes;
        let mut common = if zeroes > ones { 0 } else { 1 };
        if !most_common {
            // flip bit
            common ^= 1;
        }
        lines.retain(|line| line[col] == common);
        if lines.len() == 1 {
            return Some(
                lines
                    .get(0)?
                    .iter()
                    .fold(0, |acc, b| (acc << 1) | *b as u32),
            );
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    const DAY: u8 = 3;
    const VALID_ANSWER_PART_1: i32 = 3923414;
    const VALID_ANSWER_PART_2: u32 = 5852595;

    #[test]
    fn validate_part_1() -> Result<(), String> {
        assert_eq!(super::part1(&read_input(DAY)), VALID_ANSWER_PART_1);
        Ok(())
    }

    #[test]
    fn validate_part_2() -> Result<(), String> {
        assert_eq!(super::part2(&read_input(DAY)), VALID_ANSWER_PART_2);
        Ok(())
    }

    #[test]
    fn test_part1() {
        let test_str: &str = "000110011011
010011010000
111101010001
111101001000
111101001000
000010001110";
        super::part1(&test_str.to_string());
    }
}
