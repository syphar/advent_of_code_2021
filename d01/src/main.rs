use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<i32> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &[i32]) -> usize {
    input.windows(2).filter(|wnd| wnd[1] > wnd[0]).count()
}

fn part_2(input: &[i32]) -> usize {
    let window_sums: Vec<i32> = input.windows(3).map(|wnd| wnd.iter().sum()).collect();

    window_sums.windows(2).filter(|wnd| wnd[1] > wnd[0]).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &[i32] = &[199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    #[test]
    fn part_1_works() {
        assert_eq!(part_1(TEST_DATA), 7);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2(TEST_DATA), 5);
    }
}
