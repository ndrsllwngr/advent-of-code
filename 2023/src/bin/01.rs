use itertools::Itertools;

use aoc::read_input;

pub fn part1(input: &String) -> u64 {
    unimplemented!()
}

pub fn part2(input: &String) -> u64 {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use aoc::read_input;

    use crate::DAY;

    const VALID_ANSWER_PART_1: u64 = todo!();
    const VALID_ANSWER_PART_2: u64 = todo!();

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

const DAY: u8 = 1;


fn main() {
    println!("part1: {}", part1(&read_input(DAY)));
    println!("part2: {}", part2(&read_input(DAY)));
}