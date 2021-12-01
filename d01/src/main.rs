use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<i32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {}", part_1(input.iter()));
    println!("part 2: {}", part_2(input.iter()));
}

fn part_1<'a>(input: impl Iterator<Item = &'a i32>) -> usize {
    input
        .tuple_windows::<(&i32, &i32)>()
        .filter(|&(lhs, rhs)| rhs > lhs)
        .count()
}

fn part_2<'a>(input: impl Iterator<Item = &'a i32>) -> usize {
    input
        .tuple_windows::<(&i32, &i32, &i32)>()
        .map(|(i1, i2, i3)| i1 + i2 + i3)
        .tuple_windows::<(i32, i32)>()
        .filter(|&(lhs, rhs)| rhs > lhs)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA.iter()), 7);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_DATA.iter()), 5);
    }
}
