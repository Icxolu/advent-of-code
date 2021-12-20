use super::*;
use ndarray::prelude::*;

#[derive(Debug)]
pub struct Input {
    table: Vec<bool>,
    image: Array2<bool>,
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Result<Input, nom::Err<()>> {
    use nom::{
        character::complete::{char, line_ending},
        multi::{many1, separated_list1},
        Parser,
    };

    let element = |i| {
        char('.')
            .map(|_| false)
            .or(char('#').map(|_| true))
            .parse(i)
    };
    let table = many1(element);
    let arr = separated_list1(line_ending, many1(element)).map(|arr| {
        let rows = arr.len();
        let cols = arr[0].len();
        Array2::from_shape_vec((rows, cols), arr.into_iter().flatten().collect()).unwrap()
    });
    let mut parser = table
        .and(line_ending.and(line_ending))
        .and(arr)
        .map(|((table, _), image)| Input { table, image });
    parser.parse(input).map(|(_, i)| i)
}

fn enhance_image(
    input_image: ArrayView2<bool>,
    enhancement_table: &[bool],
    outside: bool,
) -> Array2<bool> {
    let dims = input_image.dim();
    let mut output_image = Array2::from_elem(dims, false);

    for ((i, j), v) in output_image.indexed_iter_mut() {
        let idx = [
            (i.checked_sub(1), j.checked_sub(1)),
            (i.checked_sub(1), Some(j)),
            (i.checked_sub(1), j.checked_add(1)),
            (Some(i), j.checked_sub(1)),
            (Some(i), Some(j)),
            (Some(i), j.checked_add(1)),
            (i.checked_add(1), j.checked_sub(1)),
            (i.checked_add(1), Some(j)),
            (i.checked_add(1), j.checked_add(1)),
        ]
        .into_iter()
        .fold(0, |mut value, (x, y)| {
            value <<= 1;

            let bit = match (x, y) {
                (Some(x), Some(y)) => input_image.get((x, y)).copied().unwrap_or(outside),
                _ => outside,
            };

            if bit {
                value |= 1;
            }

            value
        });

        *v = enhancement_table[idx];
    }

    output_image
}

fn calculate_step(mut img: Array2<bool>, enhancement_table: &[bool], step: usize) -> Array2<bool> {
    for step in 0..step {
        let outside = enhancement_table[0] && step % 2 != 0;
        let (rows, cols) = img.dim();
        let mut image = Array2::from_elem((rows + 2, cols + 2), outside);
        image.slice_mut(s![1..-1, 1..-1]).assign(&img);
        img = enhance_image(image.view(), enhancement_table, outside);
    }

    img
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Input) -> usize {
    calculate_step(input.image.clone(), input.table.as_slice(), 2)
        .iter()
        .filter(|v| **v)
        .count()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &Input) -> usize {
    calculate_step(input.image.clone(), input.table.as_slice(), 50)
        .iter()
        .filter(|v| **v)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#\n\n#..#.\n#....\n##..#\n..#..\n..###";

    #[test]
    fn test_part1() {
        let input = input_generator(INPUT).unwrap();
        assert_eq!(solve_part1(&input), 35);
    }

    #[test]
    fn test_part2() {
        let input = input_generator(INPUT).unwrap();
        assert_eq!(solve_part2(&input), 3351);
    }
}
