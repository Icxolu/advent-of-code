use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
pub struct FoldInstruction {
    kind: FoldInstructionKind,
    position: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum FoldInstructionKind {
    Up,
    Left,
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Result<(Vec<Point>, Vec<FoldInstruction>), nom::Err<()>> {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, line_ending, u32},
        combinator::value,
        multi::separated_list1,
        sequence::separated_pair,
        Parser,
    };

    let point = separated_pair(u32, char(','), u32).map(|(x, y)| Point {
        x: x as usize,
        y: y as usize,
    });
    let points = separated_list1(line_ending, point);

    let instruction = separated_pair(
        tag("fold along ").and(
            value(FoldInstructionKind::Left, char('x'))
                .or(value(FoldInstructionKind::Up, char('y'))),
        ),
        char('='),
        u32,
    )
    .map(|((_, kind), position)| FoldInstruction {
        kind,
        position: position as usize,
    });
    let instructions = separated_list1(line_ending, instruction);

    let mut parser = separated_pair(points, line_ending.and(line_ending), instructions);

    parser.parse(input).map(|(_, i)| i)
}

#[aoc(day13, part1)]
pub fn solve_part1((points, instructions): &(Vec<Point>, Vec<FoldInstruction>)) -> usize {
    let instruction = instructions.get(0).unwrap();

    let mut points: Vec<_> = points
        .iter()
        .cloned()
        .map(|mut point| {
            match instruction.kind {
                FoldInstructionKind::Left if point.x > instruction.position => {
                    point.x -= 2 * (point.x - instruction.position)
                }
                FoldInstructionKind::Up if point.y > instruction.position => {
                    point.y -= 2 * (point.y - instruction.position)
                }
                _ => {}
            };
            point
        })
        .collect();

    points.sort_unstable();
    points.dedup();

    points.len()
}

#[aoc(day13, part2)]
pub fn solve_part2(
    (points, instructions): &(Vec<Point>, Vec<FoldInstruction>),
) -> ndarray::Array2<usize> {
    let mut points = points.clone();
    for instruction in instructions {
        points = points
            .into_iter()
            .map(|mut point| {
                match instruction.kind {
                    FoldInstructionKind::Left if point.x > instruction.position => {
                        point.x -= 2 * (point.x - instruction.position)
                    }
                    FoldInstructionKind::Up if point.y > instruction.position => {
                        point.y -= 2 * (point.y - instruction.position)
                    }
                    _ => {}
                };
                point
            })
            .collect();

        points.sort_unstable();
        points.dedup();
    }

    let i = points.iter().map(|p| p.x).max().unwrap();
    let j = points.iter().map(|p| p.y).max().unwrap();

    points.into_iter().fold(
        ndarray::Array2::from_elem((j + 1, i + 1), 0),
        |mut arr, p| {
            arr[(p.y, p.x)] = 1;
            arr
        },
    )
}

#[cfg(test)]
mod tests {
    use std::ops::AddAssign;

    use ndarray::{s, Array2};

    use super::*;

    static INPUT: &str = "6,10\n0,14\n9,10\n0,3\n10,4\n4,11\n6,0\n6,12\n4,1\n0,13\n10,12\n3,4\n3,0\n8,4\n1,10\n2,14\n8,10\n9,0\n\nfold along y=7\nfold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(&input_generator(INPUT).unwrap()), 17);
    }

    #[test]
    fn test_part2() {
        let mut arr = Array2::from_elem((5, 5), 0);
        arr.slice_mut(s![.., 0]).add_assign(1);
        arr.slice_mut(s![.., -1]).add_assign(1);
        arr.slice_mut(s![0, 1..-1]).add_assign(1);
        arr.slice_mut(s![-1, 1..-1]).add_assign(1);

        assert_eq!(solve_part2(&input_generator(INPUT).unwrap()), arr)
    }
}
