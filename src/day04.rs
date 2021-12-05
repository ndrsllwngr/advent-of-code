use std::collections::HashSet;

fn init_winning_combos() -> [HashSet<usize>; 12] {
    [
        HashSet::from([0, 1, 2, 3, 4]),
        HashSet::from([5, 6, 7, 8, 9]),
        HashSet::from([10, 11, 12, 13, 14]),
        HashSet::from([15, 16, 17, 18, 19]),
        HashSet::from([20, 21, 22, 23, 24]),
        HashSet::from([0, 5, 10, 15, 20]),
        HashSet::from([1, 6, 11, 16, 21]),
        HashSet::from([2, 7, 12, 17, 22]),
        HashSet::from([3, 8, 13, 18, 23]),
        HashSet::from([4, 9, 14, 19, 24]),
        HashSet::from([0, 6, 12, 18, 24]),
        HashSet::from([4, 8, 12, 16, 20]),
    ]
}

pub fn part1(input: &String) -> i64 {
    let winning_combons: [HashSet<usize>; 12] = init_winning_combos();
    let (nums, mut boards) = parse_nums_and_boards(input);
    let mut winner = None;

    'outer: for num in nums {
        for (board, matches) in &mut boards {
            if let Some((idx, _)) = board.iter().enumerate().find(|(_, n)| **n == num) {
                matches.insert(idx);
            }
        }

        for (board, matches) in boards.iter() {
            for combo in winning_combons.iter() {
                if matches.is_superset(combo) {
                    winner = Some((num, board.clone(), matches.clone()));
                    break 'outer;
                }
            }
        }
    }

    if let Some((num, board, matches)) = winner {
        num * score_board(&board, &matches)
    } else {
        0
    }
}

pub fn part2(input: &String) -> i64 {
    let winning_combons: [HashSet<usize>; 12] = init_winning_combos();
    let (nums, mut boards) = parse_nums_and_boards(input);
    let mut winner = None;

    'outer: for num in nums {
        for (board, matches) in &mut boards {
            if let Some((idx, _)) = board.iter().enumerate().find(|(_, n)| **n == num) {
                matches.insert(idx);
            }
        }

        winner = Some((num, boards[0].0.clone(), boards[0].1.clone()));

        boards = boards
            .drain(..)
            .filter(|(_, matches)| {
                for combo in winning_combons.iter() {
                    if matches.is_superset(combo) {
                        return false;
                    }
                }
                true
            })
            .collect();

        if boards.is_empty() {
            break 'outer;
        }
    }

    if let Some((num, board, matches)) = winner {
        num * score_board(&board, &matches)
    } else {
        0
    }
}

fn parse_nums_and_boards(
    input: &String,
) -> (
    impl Iterator<Item = i64> + '_,
    Vec<(Vec<i64>, HashSet<usize>)>,
) {
    let mut iter = input.split("\n\n");
    let nums = iter
        .next()
        .unwrap()
        .split(',')
        .map(|n| n.parse::<i64>().unwrap());

    let boards = iter
        .map(|b| {
            b.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|board| (board, HashSet::<usize>::new()))
        .collect::<Vec<_>>();
    (nums, boards)
}

fn score_board(board: &[i64], matches: &HashSet<usize>) -> i64 {
    board
        .iter()
        .enumerate()
        .filter(|(i, _)| !matches.contains(i))
        .map(|(_, n)| n)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::read_input;
    const DAY: u8 = 4;
    const VALID_ANSWER_PART_1: i64 = 41503;
    const VALID_ANSWER_PART_2: i64 = 3178;

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
