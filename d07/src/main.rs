use std::fs;
fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn part_1(input: &[i32]) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max)
        .map(|pos| input.iter().map(|crab| (crab - pos).abs()).sum::<i32>())
        .min()
        .unwrap()
}

fn part_2(input: &[i32]) -> i32 {
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    let mut costs: Vec<usize> = vec![0];
    let mut sum: usize = 0;
    for i in 1..=max as usize {
        sum += i;
        costs.push(sum);
    }

    (min..=max)
        .map(|pos| {
            input
                .iter()
                .map(|crab| costs[(crab - pos).abs() as usize])
                .sum::<usize>()
        })
        .min()
        .unwrap()
        .try_into()
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
    #[test]
    fn test_part_2() {
        assert_eq!(part_2(&TEST_DATA), 168);
    }
}
