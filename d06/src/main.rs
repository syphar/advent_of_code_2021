use std::fs;
fn main() {
    let input: Vec<u16> = fs::read_to_string("input.txt")
        .unwrap()
        .split(',')
        .map(str::trim)
        .map(|s| s.parse().unwrap())
        .collect();

    println!("part 1: {}", calc(&input, 80));
    println!("part 2: {}", calc(&input, 256));
}

fn calc(input: &[u16], days: u16) -> usize {
    let mut fish: Vec<u16> = input.to_vec();
    for _ in 1..=days {
        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish.push(8);
                fish[i] = 6;
            } else {
                fish[i] -= 1;
            }
        }
    }

    fish.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static NUMBERS: [u16; 5] = [3, 4, 3, 1, 2];

    #[test]
    fn test_part_1() {
        assert_eq!(calc(&NUMBERS, 80), 5934);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(calc(&NUMBERS, 256), 26_984_457_539);
    }
}
