use super::*;

fn explore_basin((r, c): (usize, usize), map: &mut [Vec<bool>]) -> usize {
    let mut to_visit = vec![(r, c)];
    let mut size = 0;

    while !to_visit.is_empty() {
        let (r, c) = to_visit.pop().unwrap();

        if !map[r][c] {
            continue;
        }

        size += 1;
        map[r][c] = false;

        if let Some(true) = r.checked_sub(1).map(|r| map[r][c]) {
            to_visit.push((r - 1, c))
        }

        if let Some(true) = map.get(r + 1).map(|r| r[c]) {
            to_visit.push((r + 1, c))
        }

        if let Some(true) = c.checked_sub(1).map(|c| map[r][c]) {
            to_visit.push((r, c - 1))
        }

        if let Some(true) = map[r].get(c + 1).copied() {
            to_visit.push((r, c + 1))
        }
    }

    size
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Result<Vec<Vec<u32>>, nom::Err<()>> {
    use nom::{
        bytes::complete::take,
        character::complete::{line_ending, u32},
        multi::{many1, separated_list1},
        Parser,
    };
    let height = take(1u8).and_then(u32);
    let row = many1(height);
    let mut parser = separated_list1(line_ending, row);

    parser.parse(input).map(|(_, i)| i)
}

fn low_points(map: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let rows = map.len();
    let cols = map[0].len();

    let mut low_points = Vec::new();

    for r in 0..rows {
        for c in 0..cols {
            let current = map[r][c];

            let top = r.checked_sub(1).map(|r| map[r][c]);
            let bot = map.get(r + 1).map(|r| r[c]);
            let left = c.checked_sub(1).map(|c| map[r][c]);
            let right = map[r].get(c + 1).copied();

            if top.map_or(true, |top| top > current)
                && bot.map_or(true, |bot| bot > current)
                && left.map_or(true, |left| left > current)
                && right.map_or(true, |right| right > current)
            {
                low_points.push((r, c));
            }
        }
    }

    low_points
}

#[aoc(day9 part1)]
pub fn solve_part1(map: &[Vec<u32>]) -> u32 {
    low_points(map).iter().map(|(r, c)| map[*r][*c] + 1).sum()
}

#[aoc(day9 part2)]
pub fn solve_part2(map: &[Vec<u32>]) -> usize {
    let mut simple_map: Vec<Vec<_>> = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|&v| if v != 9 { true } else { false })
                .collect()
        })
        .collect();

    let mut basin_sizes: Vec<_> = low_points(map)
        .iter()
        .map(|&starting_point| explore_basin(starting_point, &mut simple_map))
        .collect();

    basin_sizes.sort();
    basin_sizes.iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "2199943210\n3987894921\n9856789892\n8767896789\n9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT).unwrap()), 15)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(&input_generator(INPUT).unwrap()), 1134)
    }
}
