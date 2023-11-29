pub fn part1(input: &String) -> u16 {
    let mut prev_measurement: Option<u16> = None;
    let mut count_increased = 0;
    for line in input.lines() {
        let depth: u16 = line.parse().unwrap();
        if let Some(prev_depth) = prev_measurement {
            if depth > prev_depth {
                count_increased += 1;
            }
        }
        prev_measurement = Some(depth);
    }
    count_increased
}

pub fn part2(input: &String) -> u16 {
    let mut sliding_window: Vec<u16> = Vec::new();
    let mut count_increased = 0;
    for line in input.lines() {
        let depth: u16 = line.parse().unwrap();
        if sliding_window.len() < 3 {
            sliding_window.push(depth);
        } else {
            let prev_sum: u16 = sliding_window.iter().sum();
            sliding_window.remove(0);
            sliding_window.push(depth);
            let curr_sum: u16 = sliding_window.iter().sum();

            if curr_sum > prev_sum {
                count_increased += 1;
            }
        }
    }
    count_increased
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    use crate::year2021::YEAR;

    const DAY: u8 = 1;
    const VALID_ANSWER_PART_1: u16 = 1791;
    const VALID_ANSWER_PART_2: u16 = 1822;

    #[test]
    fn validate_part_1() -> Result<(), String> {
        assert_eq!(super::part1(&read_input(YEAR,DAY)), VALID_ANSWER_PART_1);
        Ok(())
    }

    #[test]
    fn validate_part_2() -> Result<(), String> {
        assert_eq!(super::part2(&read_input(YEAR, DAY)), VALID_ANSWER_PART_2);
        Ok(())
    }
}
