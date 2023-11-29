use itertools::Itertools;

pub fn part1(input: &String) -> u64 {
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

pub fn part2(input: &String) -> u64 {
    let mut list: Vec<u64> = Vec::new();
    let mut iter = input.split("\n\n");
    let _ = iter.map(|b| {
        let mut tmp_count: u64 = 0;
        for l in b.lines() {
            let number: u64 = l.parse().unwrap();
            tmp_count = tmp_count + number;
        }
        list.push(tmp_count);
        tmp_count
    }).collect::<Vec<u64>>();
    list.sort_by(|a, b| b.cmp(a));
    list.iter().take(3).sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    use crate::year2022::YEAR;

    const DAY: u8 = 1;
    const VALID_ANSWER_PART_1: u64 = 70369;
    const VALID_ANSWER_PART_2: u16 = 203002;

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
