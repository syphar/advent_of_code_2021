use std::fs;
fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", part_1(&input));
}

fn part_1(input: &[i32]) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max)
        .map(|pos| input.iter().map(|crab| (crab - pos).abs()).sum::<i32>())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(&TEST_DATA), 37);
    }
}
