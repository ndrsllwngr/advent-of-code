use std::collections::HashSet;

pub fn unique_numbers() -> HashSet<i32> {
    let mut unique_numbers = HashSet::new();
    unique_numbers.insert(2); // 1
    unique_numbers.insert(4); // 4
    unique_numbers.insert(3); // 7
    unique_numbers.insert(7); // 8
    unique_numbers
}

pub fn part1(input: &String) -> i64 {
    let mut count = 0;
    let unique_numbers = unique_numbers();
    for line in input.lines() {
        let output_values: Vec<&str> =  line
            .trim()
            .split('|')
            .last()
            .unwrap()
            .trim()
            .split(' ')
            .collect();
        for val in output_values {
            if unique_numbers.contains(&(val.len() as i32)) {
                count += 1;
            }
        }
    }
    count
}

pub fn part2(input: &String) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    const DAY: u8 = 8;
    const VALID_ANSWER_PART_1: i64 = 278;
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
