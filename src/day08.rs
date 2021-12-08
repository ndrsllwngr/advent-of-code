pub fn part1(input: &String) -> i64 {
    0
}

pub fn part2(input: &String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    const DAY: u8 = 8;
    const VALID_ANSWER_PART_1: i64 = 0;
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
