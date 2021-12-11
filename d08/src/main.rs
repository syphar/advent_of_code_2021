use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone)]
struct Digit(String);

struct Input(Vec<(Vec<Digit>, Vec<Digit>)>);

fn main() {
    let input = parse_input(
        BufReader::new(File::open("input.txt").unwrap())
            .lines()
            .map(|l| l.unwrap()),
    );

    println!("part 1: {}", part_1(input));
}

fn parse_input<I, L>(lines: I) -> Input
where
    L: AsRef<str>,
    I: Iterator<Item = L>,
{
    Input(
        lines
            .map(|line| {
                let (lhs, rhs) = line.as_ref().split_once('|').unwrap();
                (
                    lhs.split(' ').map(|s| Digit(s.to_string())).collect(),
                    rhs.split(' ').map(|s| Digit(s.to_string())).collect(),
                )
            })
            .collect(),
    )
}

fn part_1(input: Input) -> usize {
    input
        .0
        .iter()
        .map(|l| l.1.clone())
        .flatten()
        .filter(|digit| matches!(digit.0.len(), 2 | 4 | 3 | 7))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &[&str] = &[
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe",
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc",
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg",
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea",
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb",
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe",
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef",
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb",
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    ];

    #[test]
    fn test_part_1() {
        let input = parse_input(INPUT.iter());

        assert_eq!(part_1(input), 26);
    }
}
