use itertools::Itertools;

use aoc::read_input;

const DAY: u8 = 2;

pub fn part1(input: &str) -> u64 {
    unimplemented!()
}

pub fn part2(input: &str) -> u64 {
    unimplemented!()
}


fn main() {
    let input = read_input(DAY);
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
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