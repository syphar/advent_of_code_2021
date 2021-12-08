use std::fs;
fn main() {
    let input: Vec<i32> = fs::read_to_string("input.txt")
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", calc(&input, cost_part_1));
    println!("part 2: {}", calc(&input, cost_part_2));
}

fn cost_part_1(distance: i32) -> i32 {
    distance
}

fn cost_part_2(distance: i32) -> i32 {
    // too direct for real math,
    // formula from:
    // https://www.wolframalpha.com/input/?i=0%2C1%2C3%2C6%2C10%2C15%2C21%2C28%2C36%2C45%2C55%2C66%2C78%2C91%2C105%2C120%2C136%2C..
    (0.5 * distance as f32 * (distance + 1) as f32) as i32
}

fn calc<F>(input: &[i32], cost_fn: F) -> i32
where
    F: Fn(i32) -> i32,
{
    let min = *input.iter().min().unwrap();
    let max = *input.iter().max().unwrap();

    (min..=max)
        .map(|pos| {
            input
                .iter()
                .map(|crab| cost_fn((crab - pos).abs()))
                .sum::<i32>()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DATA: [i32; 10] = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    #[test]
    fn test_part_1() {
        assert_eq!(calc(&TEST_DATA, cost_part_1), 37);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(calc(&TEST_DATA, cost_part_2), 168);
    }
}
