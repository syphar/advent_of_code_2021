use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SomeError {
    #[error("could not parse")]
    ParseError,
}

struct Board {
    numbers: [u8; 25],
    found: [bool; 25],
    last_called: Option<u8>,
}

impl Board {
    fn coord(x: u8, y: u8) -> usize {
        (x * 5 + y).into()
    }

    fn set(&mut self, number: u8) {
        for i in 0..25 {
            if self.numbers[i] == number {
                self.found[i] = true;
            }
        }
        self.last_called = Some(number);
    }

    fn has_won(&self) -> bool {
        for x in 0..5 {
            if (0..5).all(|y| self.found[Self::coord(x, y)]) {
                return true;
            }
        }
        for y in 0..5 {
            if (0..5).all(|x| self.found[Self::coord(x, y)]) {
                return true;
            }
        }

        false
    }

    fn score(&self) -> u16 {
        self.numbers
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.found[*i])
            .map(|(_, n)| *n as u16)
            .sum::<u16>()
            * (self.last_called.unwrap() as u16)
    }
}

impl FromStr for Board {
    type Err = SomeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut b = Board {
            numbers: [0; 25],
            found: [false; 25],
            last_called: None,
        };
        for (l, line) in s.trim().lines().enumerate() {
            for (i, num) in line.trim().split(' ').filter(|s| !s.is_empty()).enumerate() {
                let num: u8 = num.parse().unwrap();

                let idx = Self::coord(u8::try_from(l).unwrap(), u8::try_from(i).unwrap());
                b.numbers[idx] = num;
                b.found[idx] = false;
            }
        }
        Ok(b)
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..5 {
            for y in 0..5 {
                let idx = Self::coord(x, y);
                let num = format!("{:>2}", self.numbers[idx]);
                if self.found[idx] {
                    write!(f, "[{}] ", num)?;
                } else {
                    write!(f, " {}  ", num)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let mut content = String::new();
    File::open("input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let numbers: Vec<u8> = content
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards: Vec<Board> = content
        .split("\n\n")
        .skip(2)
        .map(|s| Board::from_str(s).unwrap())
        .collect();

    println!("result part 1: {}", part_1(numbers, &mut boards))
}

fn part_1(numbers: impl IntoIterator<Item = u8>, boards: &mut Vec<Board>) -> u16 {
    for n in numbers {
        for b in boards.iter_mut() {
            b.set(n);
            if b.has_won() {
                return b.score();
            }
        }
    }
    panic!("no board has won!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS: [u8; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    const BOARDS: &str = "
        22 13 17 11  0
         8  2 23  4 24
        21  9 14 16  7
         6 10  3 18  5
         1 12 20 15 19

         3 15  0  2 22
         9 18 13 17  5
        19  8  7 25 23
        20 11 10 24  4
        14 21 16 12  6

        14 21 17 24  4
        10 16 15  9 19
        18  8 23 26 20
        22 11 13  6  5
         2  0 12  3  7
    ";

    #[test]
    fn test_part_1() {
        let mut boards: Vec<Board> = BOARDS
            .split("\n\n")
            .map(|s| Board::from_str(s).unwrap())
            .collect();

        assert_eq!(part_1(NUMBERS, &mut boards), 4512);
    }
}
