use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<String> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| line.unwrap())
        .collect();

    println!("part 1: {}", part_1::<_, _, 12>(input.iter()));
    // println!("part 2: {}", part_2(input.iter()));
}
// pub fn from_text<T>(style: Option<Style>, value: T) -> Vec<Segment>
// where
//     T: Into<String>,
// {

fn part_1<T, I, const N: usize>(input: I) -> i32
where
    T: AsRef<str>,
    I: Iterator<Item = T>,
{
    let mut count_1 = vec![0; N];
    let mut count_0 = vec![0; N];

    for line in input {
        for (i, ch) in line.as_ref().chars().enumerate() {
            match ch {
                '1' => count_1[i] += 1,
                '0' => count_0[i] += 1,
                _ => panic!("unknown character"),
            }
        }
    }

    let mut gamma = vec!['0'; N];
    let mut epsilon = vec!['0'; N];

    for i in 0..N {
        if count_1[i] > count_0[i] {
            gamma[i] = '1'
        }
        if count_1[i] < count_0[i] {
            epsilon[i] = '1'
        }
    }

    let gamma = i32::from_str_radix(&gamma.iter().collect::<String>(), 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon.iter().collect::<String>(), 2).unwrap();

    gamma * epsilon
}

// fn part_2<T: AsRef<str>>(input: impl Iterator<Item = T>) -> i32 {
//     todo!();
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[&str] = &[
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];

    #[test]
    fn part_1_works() {
        assert_eq!(part_1::<_, _, 5>(TEST_DATA.iter()), 198);
    }

    // #[test]
    // fn part_2_works() {
    //     assert_eq!(part_2(TEST_DATA.iter()), 900);
    // }
}
