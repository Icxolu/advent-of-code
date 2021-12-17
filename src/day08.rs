use super::*;
use enumflags2::{bitflags, BitFlags};
use std::collections::HashMap;

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Debug, Clone)]
pub struct Signal {
    input: Vec<String>,
    output: Vec<String>,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Signal> {
    input
        .lines()
        .map(|l| {
            let (input, output) = l.split_once('|').unwrap();
            let input: Vec<_> = input
                .split_whitespace()
                .map(|signal| signal.to_owned())
                .collect();

            let output: Vec<_> = output
                .split_whitespace()
                .map(|segments| segments.to_owned())
                .collect();

            Signal { input, output }
        })
        .collect()
}

#[aoc(day8 part1)]
pub fn solve_part1(signals: &[Signal]) -> usize {
    signals
        .iter()
        .flat_map(|s| &s.output)
        .filter_map(|o| match o.len() {
            l @ (2 | 3 | 4 | 7) => Some(l),
            _ => None,
        })
        .count()
}

fn segments(pattern: &str) -> BitFlags<Segment> {
    let bits = pattern.bytes().fold(0u8, |bits, b| {
        let b = b - b'a';
        bits | 1 << b
    });
    BitFlags::<Segment>::from_bits(bits).unwrap()
}

#[aoc(day8 part2)]
pub fn solve_part2(signals: &[Signal]) -> u32 {
    let mut sum = 0;
    for signal in signals {
        let Signal { input, output } = signal;
        let mut digits: HashMap<usize, _> = HashMap::new();
        for pattern in input {
            let flags = segments(pattern);

            match pattern.len() {
                2 => {
                    digits.insert(1, flags);
                }
                3 => {
                    digits.insert(7, flags);
                }
                4 => {
                    digits.insert(4, flags);
                }
                7 => {
                    digits.insert(8, flags);
                }
                _ => continue,
            };
        }

        let num = output
            .iter()
            .map(|digit| {
                let flags = segments(digit);
                match digit.len() {
                    2 => 1,
                    3 => 7,
                    4 => 4,
                    7 => 8,
                    5 => {
                        if flags.contains(digits[&7]) {
                            3
                        } else if flags.contains(digits[&4] ^ digits[&1]) {
                            5
                        } else {
                            2
                        }
                    }
                    6 => {
                        if !flags.contains(digits[&1]) {
                            6
                        } else if flags.contains(digits[&4]) {
                            9
                        } else {
                            0
                        }
                    }
                    _ => unreachable!(),
                }
            })
            .rev()
            .enumerate()
            .fold(0, |num, (i, digit)| num + 10_u32.pow(i as u32) * digit);

        sum += num;
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe \nedbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc \nfgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg \nfbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb \naecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea \nfgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb \ndbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe \nbdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef \negadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb \ngcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT)), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT)), 61229)
    }
}
