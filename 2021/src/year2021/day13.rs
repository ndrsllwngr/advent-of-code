use std::collections::HashSet;

use itertools::Itertools;

use crate::list_to_vec;

type Point = (i32, i32);
type Fold = (char, i32);

fn parse_input(input: &String) -> (Vec<Point>, Vec<Fold>) {
    let mut split = input.split("\n\n");
    let points = split.next().unwrap().lines().map(|l| {
        let mut c_split = l.split(',').map(|c| c.parse::<i32>().unwrap());
        (c_split.next().unwrap(), c_split.next().unwrap())
    }).collect::<Vec<Point>>();
    let instructions = split.next().unwrap().lines().map(|l| {
        let mut i_split = l.strip_prefix("fold along ").unwrap().split('=');
        (i_split.next().unwrap().chars().next().unwrap(), i_split.next().unwrap().parse::<i32>().unwrap())
    }).collect::<Vec<Fold>>();
    (points, instructions)
}

fn fold(f: Fold, set: &mut HashSet<Point>) {
    let fold_coordinate = f.1;

    let compare = match f.0 {
        'x' => |f, (x, _)| x > f,
        'y' => |f, (_, y)| y > f,
        _ => panic!("fold char={}", f.0)
    };
    let fold_closure = match f.0 {
        'x' => |f, (x, y)| (2 * f - x, y),
        'y' => |f, (x, y)| (x, 2 * f - y),
        _ => panic!("fold char={}", f.0)
    };

    set
        .clone()
        .iter()
        .filter(|&p| compare(fold_coordinate, *p))
        .for_each(|p| {
            set.remove(p);
            set.insert(fold_closure(fold_coordinate, *p));
        });
}

pub fn part1(input: &String) -> u32 {
    let mut set: HashSet<Point> = HashSet::new();
    let (points, instructions) = parse_input(input);
    points
        .iter()
        .for_each(|p| { set.insert(*p); });

    fold(instructions[0], &mut set);
    set
        .iter()
        .count() as u32
}

pub fn part2(input: &String) -> u32 {
    let mut set: HashSet<Point> = HashSet::new();
    let (points, instructions) = parse_input(input);
    points
        .iter()
        .for_each(|p| { set.insert(*p); });

    instructions
        .iter()
        .for_each(|i| fold(*i, &mut set));

    let (x_min, x_max) = (set.iter().map(|(x,_)| *x).min().unwrap(), set.iter().map(|(x,_)| *x).max().unwrap());
    let (y_min, y_max) = (set.iter().map(|(y,_)| *y).min().unwrap(), set.iter().map(|(y,_)| *y).max().unwrap());

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            print!("{}", set.contains(&(x,y)).then(|| '█').or_else(|| Some(' ')).unwrap())
        }
        println!();
    }

    set
        .iter()
        .count() as u32
}


#[cfg(test)]
mod tests {
    use crate::read_input;
    use crate::year2021::YEAR;

    const DAY: u8 = 13;
    const VALID_ANSWER_PART_1: u32 = 781;
    const VALID_ANSWER_PART_2: u32 = 99; // PERCGJPB

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
