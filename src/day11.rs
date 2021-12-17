use super::*;
use ndarray::prelude::*;
use std::ops::AddAssign;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Result<Array2<u32>, nom::Err<()>> {
    use nom::{
        character::complete::{line_ending, satisfy},
        multi::{many1, separated_list1},
        Parser,
    };

    let digit = satisfy(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap());
    let line = many1(digit);
    let mut parser = separated_list1(line_ending, line)
        .map(|v| Array::from_shape_vec((10, 10), v.into_iter().flatten().collect()).unwrap());

    parser.parse(input).map(|(_, p)| p)
}

fn perform_step(mut input: ArrayViewMut2<u32>) -> usize {
    input += 1;
    let mut has_flashed = Array2::from_elem((10, 10), false);
    loop {
        let pos: Vec<_> = input
            .indexed_iter()
            .filter_map(|(pos, v)| {
                if *v > 9 && !has_flashed[pos] {
                    Some(pos)
                } else {
                    None
                }
            })
            .collect();

        if pos.is_empty() {
            break;
        }

        for (x, y) in pos.into_iter() {
            let x_min = x.checked_sub(1).unwrap_or(x);
            let y_min = y.checked_sub(1).unwrap_or(y);

            let x_max = (x + 1).min(9);
            let y_max = (y + 1).min(9);

            input
                .slice_mut(s![x_min..=x_max, y_min..=y_max])
                .add_assign(1);
            has_flashed[(x, y)] = true;
        }
    }

    input.iter_mut().filter(|v| **v > 9).map(|v| *v = 0).count()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &Array2<u32>) -> usize {
    let mut input = input.clone();
    (0..100).map(|_| perform_step(input.view_mut())).sum()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &Array2<u32>) -> usize {
    let mut input = input.clone();
    let mut i = 0;

    loop {
        let flashes = perform_step(input.view_mut());
        i += 1;
        if flashes == 100 {
            break i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "5483143223\n2745854711\n5264556173\n6141336146\n6357385478\n4167524645\n2176841721\n6882881134\n4846848554\n5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT).unwrap()), 1656)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT).unwrap()), 195)
    }
}
