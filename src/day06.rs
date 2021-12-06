use std::collections::HashMap;

fn calculate_population(initial_fish: Vec<i64>, days: i64) -> i64 {
    let mut population: HashMap<i64, i64> = HashMap::new();
    for fish in initial_fish {
        *population.entry(fish).or_insert(0) += 1;
    }
    for _ in 1..=days {
        let mut new_population: HashMap<i64, i64> = HashMap::new();
        for age in population.keys() {
            if let Some(fish_count) = population.get(age) {
                if *age != 0 {
                    *new_population.entry(*age - 1).or_insert(0) += *fish_count;
                } else {
                    *new_population.entry(6).or_insert(0) += *fish_count;
                    *new_population.entry(8).or_insert(0) += *fish_count;
                }
            }
        }
        population = new_population;
    }
    population.values().sum()
}

pub fn part1(input: &String) -> i64 {
    let initial_fish: Vec<i64> = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    const DAYS: i64 = 80;
    calculate_population(initial_fish, DAYS)
}

pub fn part2(input: &String) -> i64 {
    let initial_fish: Vec<i64> = input
        .split(',')
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    const DAYS: i64 = 256;
    calculate_population(initial_fish, DAYS)
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    const DAY: u8 = 6;
    const VALID_ANSWER_PART_1: i64 = 353274;
    const VALID_ANSWER_PART_2: i64 = 1609314870967;

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
