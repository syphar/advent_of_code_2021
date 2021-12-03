use bitvec::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let input: Vec<_> = BufReader::new(File::open("input.txt").unwrap())
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| (c == '1'))
                .collect::<BitVec<Msb0>>()
        })
        .collect();

    println!("part 1: {}", part_1::<_, _, 12>(input.iter()));
    println!("part 2: {}", part_2::<_, _, 12>(input.iter()));
}

fn part_1<T, I, const N: usize>(input: I) -> usize
where
    T: AsRef<BitSlice<Msb0>>,
    I: Iterator<Item = T>,
{
    let mut count_1 = [0; N];
    let mut count_0 = [0; N];

    for bytes in input {
        for i in 0..N {
            if bytes.as_ref()[i] {
                count_1[i] += 1;
            } else {
                count_0[i] += 1;
            }
        }
    }

    let mut gamma = bitvec![Msb0, usize;0;N];
    let mut epsilon = bitvec![Msb0, usize;0;N];

    for i in 0..N {
        if count_1[i] > count_0[i] {
            gamma.set(i, true);
        }
        if count_1[i] < count_0[i] {
            epsilon.set(i, true);
        }
    }

    let gamma = gamma.load_le::<usize>();
    let epsilon = epsilon.load_le::<usize>();

    gamma * epsilon
}

fn part_2<T, I, const N: usize>(input: I) -> usize
where
    T: AsRef<BitSlice<Msb0>>,
    I: Iterator<Item = T>,
{
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitvec::prelude::*;

    const TEST_DATA: [BitArray<Msb0>; 12] = [
        bitarr![const Msb0, usize; 0, 0, 1, 0, 0],
        bitarr![const Msb0, usize; 1, 1, 1, 1, 0],
        bitarr![const Msb0, usize; 1, 0, 1, 1, 0],
        bitarr![const Msb0, usize; 1, 0, 1, 1, 1],
        bitarr![const Msb0, usize; 1, 0, 1, 0, 1],
        bitarr![const Msb0, usize; 0, 1, 1, 1, 1],
        bitarr![const Msb0, usize; 0, 0, 1, 1, 1],
        bitarr![const Msb0, usize; 1, 1, 1, 0, 0],
        bitarr![const Msb0, usize; 1, 0, 0, 0, 0],
        bitarr![const Msb0, usize; 1, 1, 0, 0, 1],
        bitarr![const Msb0, usize; 0, 0, 0, 1, 0],
        bitarr![const Msb0, usize; 0, 1, 0, 1, 0],
    ];

    #[test]
    fn part_1_works() {
        assert_eq!(part_1::<_, _, 5>(TEST_DATA.iter()), 198);
    }

    #[test]
    fn part_2_works() {
        assert_eq!(part_2::<_, _, 5>(TEST_DATA.iter()), 230);
    }
}
