use crate::list_to_vec;
use itertools::Itertools;

pub fn part1(input: &String) -> i64 {
    let mut start_positions = list_to_vec::<i64>(input);
    // optimal would be to calculate geometric median
    // but regular median suffices for this input
    start_positions.sort_unstable();
    let target = start_positions.get(start_positions.len() / 2).unwrap();
    start_positions
        .iter()
        .map(|pos| (target - pos).abs())
        .sum()
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

    const DAY: u8 = 7;
    const VALID_ANSWER_PART_1: i64 = 349357;
    const VALID_ANSWER_PART_2: i64 = 96708205;

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
