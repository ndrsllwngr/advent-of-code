use std::cmp::{max, min};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    x: i64,
    y: i64,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(',').collect();
        let x_from = coords[0].parse::<i64>()?;
        let y_from = coords[1].parse::<i64>()?;
        Ok(Point {
            x: x_from,
            y: y_from,
        })
    }
}

pub struct Line {
    a: Point,
    b: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(" -> ");
        let a_from: Point = Point::from_str(iter.next().unwrap()).unwrap();
        let b_from: Point = Point::from_str(iter.next().unwrap()).unwrap();
        Ok(Line {
            a: a_from,
            b: b_from,
        })
    }
}

impl Line {
    fn between(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();
        if self.is_horizontal() {
            for y in min(self.a.y, self.b.y)..=max(self.a.y, self.b.y) {
                points.push(Point { x: self.a.x, y });
            }
        } else if self.is_vertical() {
            for x in min(self.a.x, self.b.x)..=max(self.a.x, self.b.x) {
                points.push(Point { x, y: self.a.y });
            }
        }
        points
    }

    fn is_horizontal(&self) -> bool {
        self.a.x == self.b.x
    }

    fn is_vertical(&self) -> bool {
        self.a.y == self.b.y
    }

    fn is_straight(&self) -> bool {
        self.is_horizontal() || self.is_vertical()
    }

    fn is_diagonal(&self) -> bool {
        (self.b.x - self.a.x).abs() == (self.b.y - self.a.y).abs()
    }

    fn between_diagonal(&self) -> Vec<Point> {
        let mut points: Vec<Point> = Vec::new();

        let abs_diff = (self.a.x - self.b.x).abs();
        let x_direction = self.a.x - self.b.x;
        let y_direction = self.a.y - self.b.y;

        for xy in 0..=abs_diff {
            let x_coord = self.a.x + if x_direction < 0 { xy } else { xy * -1 };
            let y_coord = self.a.y + if y_direction < 0 { xy } else { xy * -1 };
            points.push(Point {
                x: x_coord,
                y: y_coord,
            });
        }
        points
    }
}

pub fn part1(input: &String) -> i64 {
    let input_lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut map: HashMap<Point, i64> = HashMap::new();

    for ln in input_lines.iter() {
        if ln.is_straight() {
            for p in ln.between().iter() {
                *map.entry(*p).or_insert(0) += 1;
            }
        }
    }

    let count: i64 = map.values().filter(|v| **v >= 2).count() as i64;
    count
}

pub fn part2(input: &String) -> i64 {
    let input_lines: Vec<Line> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut map: HashMap<Point, i64> = HashMap::new();

    for ln in input_lines.iter() {
        if ln.is_straight() {
            for p in ln.between().iter() {
                *map.entry(*p).or_insert(0) += 1;
            }
        } else if ln.is_diagonal() {
            for p in ln.between_diagonal().iter() {
                *map.entry(*p).or_insert(0) += 1;
            }
        }
    }

    let count: i64 = map.values().filter(|v| **v >= 2).count() as i64;
    count
}

#[cfg(test)]
mod tests {
    use crate::read_input;

    const DAY: u8 = 5;
    const VALID_ANSWER_PART_1: i64 = 4728;
    const VALID_ANSWER_PART_2: i64 = 17717;

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
