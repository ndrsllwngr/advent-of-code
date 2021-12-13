use std::collections::HashSet;
use itertools::Itertools;

use crate::list_to_vec;

pub fn part1(input: &String) -> u32 {
    let input: Vec<Vec<u32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let mut sum = 0;
    let rows = input.len() as i32;
    let cols = input.first().unwrap().len() as i32;
    for r in 0..rows {
        for c in 0..cols {
            let curr = input.get(r as usize).unwrap().get(c as usize).unwrap();
            if (r == 0 || curr < input.get((r - 1) as usize).unwrap().get(c as usize).unwrap())
                && (r == rows - 1 || curr < input.get((r + 1) as usize).unwrap().get(c as usize).unwrap())
                && (c == 0 || curr < input.get(r as usize).unwrap().get((c - 1) as usize).unwrap())
                && (c == cols - 1 || curr < input.get(r as usize).unwrap().get((c + 1) as usize).unwrap())
            {
                sum += curr + 1;
            }
        }
    }
    sum
}

pub fn part2(input: &String) -> i64 {
    let start_positions = list_to_vec::<i64>(input);
    let (&lo, &hi) = start_positions.iter().minmax().into_option().unwrap();
    (lo..=hi)
        .map(|p| {
            start_positions
                .iter()
                .map(|&n| (n - p).abs())
                .map(|n| n * (n + 1) / 2)
                .sum::<i64>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    const DAY: u8 = 9;
    const VALID_ANSWER_PART_1: u32 = 0;
    const VALID_ANSWER_PART_2: i64 = 0;

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
}
