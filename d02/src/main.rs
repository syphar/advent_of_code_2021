use std::fs::File;
use std::io::{BufRead, BufReader};

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn main() {
    let input: Vec<Command> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (command, number) = line.trim().split_once(" ").unwrap();
            let number: i32 = number.parse().unwrap();
            match command {
                "forward" => Command::Forward(number),
                "down" => Command::Down(number),
                "up" => Command::Up(number),
                _ => panic!("unknown command: {}", command),
            }
        })
        .collect();

    println!("part 1: {}", part_1(input.iter()));
    println!("part 2: {}", part_2(input.iter()));
}

fn part_1<'a>(input: impl Iterator<Item = &'a Command>) -> i32 {
    let mut horz = 0;
    let mut vert = 0;

    for cmd in input {
        match cmd {
            Command::Forward(n) => horz += n,
            Command::Down(n) => vert += n,
            Command::Up(n) => vert -= n,
        }
    }

    horz * vert
}

fn part_2<'a>(input: impl Iterator<Item = &'a Command>) -> i32 {
    let mut horz = 0;
    let mut vert = 0;
    let mut aim = 0;

    for cmd in input {
        match cmd {
            Command::Forward(n) => {
                horz += n;
                vert += aim * n;
            }
            Command::Down(n) => aim += n,
            Command::Up(n) => aim -= n,
        }
    }

    horz * vert
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[&Command] = &[
        &Command::Forward(5),
        &Command::Down(5),
        &Command::Forward(8),
        &Command::Up(3),
        &Command::Down(8),
        &Command::Forward(2),
    ];

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA.iter().cloned()), 150);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_DATA.iter().cloned()), 900);
    }
}
