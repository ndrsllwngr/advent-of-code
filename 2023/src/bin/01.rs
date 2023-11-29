use itertools::Itertools;

use aoc::read_example;

const DAY: u8 = 1;

pub fn part1(input: &str) -> u64 {
    let mut max_count: u64 = 0;
    let mut iter = input.split("\n\n");
    let _ = iter.map(|b| {
        let mut tmp_count: u64 = 0;
        for l in b.lines() {
            let number: u64 = l.parse().unwrap();
            tmp_count = tmp_count + number;
        }
        if max_count < tmp_count {
            max_count = tmp_count;
        }
        tmp_count
    }).collect::<Vec<u64>>();
    max_count
}

pub fn part2(input: &str) -> u64 {
    unimplemented!()
}


fn main() {
    let input = read_example(DAY);
    println!("part1: {}", part1(&input));
    //println!("part2: {}", part2(&input));
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