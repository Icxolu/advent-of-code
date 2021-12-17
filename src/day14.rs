use super::*;
use itertools::Itertools;
use std::collections::HashMap;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Result<(String, HashMap<(char, char), char>), nom::Err<()>> {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, anychar, line_ending},
        multi::separated_list1,
        sequence::separated_pair,
        Parser,
    };

    let pair_insertion = separated_pair(anychar.and(anychar), tag(" -> "), anychar);
    let pair_insertions = separated_list1(line_ending, pair_insertion);

    let mut parser = separated_pair(alpha1, line_ending.and(line_ending), pair_insertions);

    parser
        .parse(input)
        .map(|(_, (s, i))| (s.to_owned(), i.into_iter().collect()))
}

fn calculate_step(
    template: &str,
    instructions: &HashMap<(char, char), char>,
    n: usize,
) -> Option<u64> {
    let initial_map = template
        .chars()
        .tuple_windows()
        .fold(HashMap::new(), |mut map, (a, b)| {
            *map.entry((a, b)).or_insert(0) += 1;
            map
        });

    let pair_map = (0..n).fold(initial_map, |map, _| {
        map.into_iter()
            .fold(HashMap::new(), |mut map, (pair, count)| {
                if let Some(&c) = instructions.get(&pair) {
                    *map.entry((pair.0, c)).or_insert(0) += count;
                    *map.entry((c, pair.1)).or_insert(0) += count;
                } else {
                    *map.entry(pair).or_insert(0) += count;
                }

                map
            })
    });

    let count_map = pair_map
        .into_iter()
        .fold(HashMap::new(), |mut map, (pair, count)| {
            *map.entry(pair.1).or_insert(0) += count;
            map
        });

    Some(count_map.values().max()? - count_map.values().min()?)
}

#[aoc(day14, part1)]
pub fn solve_part1(
    (template, instructions): &(String, HashMap<(char, char), char>),
) -> Option<u64> {
    calculate_step(template, instructions, 10)
}

#[aoc(day14, part2)]
pub fn solve_part2(
    (template, instructions): &(String, HashMap<(char, char), char>),
) -> Option<u64> {
    calculate_step(template, instructions, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "NNCB\n\nCH -> B\nHH -> N\nCB -> H\nNH -> C\nHB -> C\nHC -> B\nHN -> C\nNN -> C\nBH -> H\nNC -> B\nNB -> B\nBN -> B\nBB -> N\nBC -> B\nCC -> N\nCN -> C";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT).unwrap()), Some(1588));
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2(&input_generator(INPUT).unwrap()),
            Some(2188189693529)
        );
    }
}
