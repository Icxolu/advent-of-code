use super::*;
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Result<Vec<u32>, nom::Err<()>> {
    use nom::{
        character::complete::{line_ending, u32},
        multi::separated_list1,
    };

    let mut parse = separated_list1(line_ending, u32);
    parse(input).map(|(_, p)| p)
}

#[aoc(day1, part1)]
pub fn solve_part1(depths: &[u32]) -> usize {
    depths.windows(2).filter(|d| d[0] < d[1]).count()
}

#[aoc(day1, part2)]
pub fn solve_part2(depths: &[u32]) -> usize {
    depths
        .windows(3)
        .map(|d| d.iter().sum())
        .tuple_windows()
        .filter(|(a, b): &(u32, u32)| a < b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(input_generator(INPUT).unwrap().as_slice()), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(input_generator(INPUT).unwrap().as_slice()), 5)
    }
}
