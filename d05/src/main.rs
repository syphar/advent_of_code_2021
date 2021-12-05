use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Hash, PartialEq, Eq, Clone)]
struct Point {
    x: u16,
    y: u16,
}
impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.trim().split_once(",").unwrap();
        Ok(Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

struct Line {
    from: Point,
    to: Point,
}

impl Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} -> {:?}", self.from, self.to)
    }
}

impl Line {
    fn covering_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            self.normalized_y()
                .map(|y| Point { x: self.from.x, y })
                .collect()
        } else if self.is_vertical() {
            self.normalized_x()
                .map(|x| Point { y: self.from.y, x })
                .collect()
        } else if self.is_diagonal() {
            let x_diff: i16 = if self.from.x > self.to.x { -1 } else { 1 };
            let y_diff: i16 = if self.from.y > self.to.y { -1 } else { 1 };

            let mut result = Vec::new();
            for i in 0i16..(self.normalized_x().len() as i16) {
                result.push(Point {
                    x: (self.from.x as i16 + ((i * x_diff) as i16)) as u16,
                    y: (self.from.y as i16 + ((i * y_diff) as i16)) as u16,
                });
            }
            result
        } else {
            panic!("unknown line construct: {:?}", self);
        }
    }

    fn is_horizontal(&self) -> bool {
        self.from.x == self.to.x
    }
    fn is_vertical(&self) -> bool {
        self.from.y == self.to.y
    }
    fn is_diagonal(&self) -> bool {
        self.normalized_x().len() == self.normalized_y().len()
    }

    fn normalized_x(&self) -> RangeInclusive<u16> {
        if self.from.x < self.to.x {
            self.from.x..=self.to.x
        } else {
            self.to.x..=self.from.x
        }
    }
    fn normalized_y(&self) -> RangeInclusive<u16> {
        if self.from.y < self.to.y {
            self.from.y..=self.to.y
        } else {
            self.to.y..=self.from.y
        }
    }
}

fn parse<I, L>(input_lines: I) -> Vec<Line>
where
    L: AsRef<str>,
    I: Iterator<Item = L>,
{
    input_lines
        .filter(|line| !line.as_ref().trim().is_empty())
        .map(|line| {
            let (from, to) = line.as_ref().split_once("->").unwrap();
            let from = Point::from_str(from).unwrap();
            let to = Point::from_str(to).unwrap();
            Line { from, to }
        })
        .collect()
}

fn part_x<'a>(lines: impl Iterator<Item = &'a Line>, only_horizontal_and_vertical: bool) -> usize {
    let mut found = HashSet::new();

    let mut dangerous = HashSet::new();

    for line in lines {
        if only_horizontal_and_vertical && !(line.is_horizontal() || line.is_vertical()) {
            continue;
        }
        for point in line.covering_points() {
            if found.contains(&point) {
                dangerous.insert(point);
            } else {
                found.insert(point);
            }
        }
    }

    dangerous.len()
}

fn main() {
    let lines: Vec<Line> = parse(
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(Result::unwrap),
    );

    println!("result part 1: {}", part_x(lines.iter(), true));
    println!("result part 2: {}", part_x(lines.iter(), false));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    const TEST_DATA: &str = "
        0,9 -> 5,9
        8,0 -> 0,8
        9,4 -> 3,4
        2,2 -> 2,1
        7,0 -> 7,4
        6,4 -> 2,0
        0,9 -> 2,9
        3,4 -> 1,4
        0,0 -> 8,8
        5,5 -> 8,2
        ";

    #[test]
    fn test_part_1() {
        let input = parse(TEST_DATA.lines());

        assert_eq!(part_x(input.iter(), true), 5);
    }

    #[test]
    fn test_part_2() {
        let input = parse(TEST_DATA.lines());

        assert_eq!(part_x(input.iter(), false), 12);
    }

    #[test_case(Line{from: Point{x: 1,y: 1}, to:Point{x: 1,y: 3}}, &[Point{x: 1,y: 1}, Point{x: 1,y: 2}, Point{x: 1,y: 3}])]
    #[test_case(Line{from: Point{x: 9,y: 7}, to: Point{x: 7,y:7}}, &[Point{x: 9,y: 7}, Point{x: 8,y: 7}, Point{x: 7,y: 7}])]
    #[test_case(Line{from: Point{x: 1,y: 1}, to: Point{x: 3,y:3}}, &[Point{x: 1,y: 1}, Point{x: 2,y: 2}, Point{x: 3,y: 3}])]
    #[test_case(Line{from: Point{x: 9,y: 7}, to: Point{x: 7,y:9}}, &[Point{x: 9,y: 7}, Point{x: 8,y: 8}, Point{x: 7,y: 9}])]
    fn point_covering(line: Line, expected: &[Point]) {
        let covered: HashSet<Point> = HashSet::from_iter(line.covering_points().iter().cloned());
        let wanted: HashSet<Point> = HashSet::from_iter(expected.iter().cloned());
        assert_eq!(covered, wanted);
    }
}
