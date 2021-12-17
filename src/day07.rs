use super::*;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Result<Vec<i32>, nom::Err<()>> {
    use nom::{
        character::complete::{char, i32},
        multi::separated_list1,
        Parser,
    };

    let mut parse = separated_list1(char(','), i32);
    parse.parse(input).map(|(_, p)| p)
}

fn find_min(crabs: &[i32], fuel: impl Fn(i32) -> i32) -> i32 {
    (crabs.iter().min().copied().unwrap()..=crabs.iter().max().copied().unwrap())
        .map(|p| fuel(p))
        .min()
        .unwrap()
}

#[aoc(day7 part1)]
pub fn solve_part1(crabs: &[i32]) -> i32 {
    find_min(crabs, |t: i32| -> i32 {
        crabs.iter().map(|pos| (pos - t).abs()).sum()
    })
}

#[aoc(day7 part2)]
pub fn solve_part2(crabs: &[i32]) -> i32 {
    find_min(crabs, |t: i32| -> i32 {
        crabs
            .iter()
            .map(|pos| {
                let delta = (pos - t).abs();
                delta * (delta + 1) / 2
            })
            .sum()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let crabs = input_generator("16,1,2,0,4,2,7,1,2,14").unwrap();
        assert_eq!(solve_part1(&crabs), 37)
    }

    #[test]
    fn test_part2() {
        let crabs = input_generator("16,1,2,0,4,2,7,1,2,14").unwrap();
        assert_eq!(solve_part2(&crabs), 168)
    }
}
