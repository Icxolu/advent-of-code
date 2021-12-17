use super::*;

#[derive(Debug, Clone)]
pub struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn intersects(&self, p: &Point) -> bool {
        let dx_l = self.end.x as i32 - self.start.x as i32;
        let dy_l = self.end.y as i32 - self.start.y as i32;

        let dx_p = p.x as i32 - self.start.x as i32;
        let dy_p = p.y as i32 - self.start.y as i32;

        let cross = dx_p * dy_l - dy_p * dx_l;

        if cross != 0 {
            return false;
        }

        if dx_l.abs() >= dy_l.abs() {
            if dx_l > 0 {
                self.start.x <= p.x && p.x <= self.end.x
            } else {
                self.end.x <= p.x && p.x <= self.start.x
            }
        } else {
            if dy_l > 0 {
                self.start.y <= p.y && p.y <= self.end.y
            } else {
                self.end.y <= p.y && p.y <= self.start.y
            }
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Result<Vec<Line>, nom::Err<()>> {
    use nom::{
        bytes::complete::tag,
        character::complete::{char, line_ending, u32},
        multi::separated_list1,
        sequence::separated_pair,
        Parser,
    };

    let coord = |i| {
        separated_pair(u32, char(','), u32)
            .map(|(x, y)| Point {
                x: x as usize,
                y: y as usize,
            })
            .parse(i)
    };
    let line = separated_pair(coord, tag(" -> "), coord).map(|(start, end)| Line { start, end });
    let mut parse = separated_list1(line_ending, line);

    parse.parse(input).map(|(_, p)| p)
}

fn calculate_intersection(mut lines: Vec<Line>, filter: impl FnMut(&Line) -> bool) -> usize {
    lines.retain(filter);

    let max_x = lines
        .iter()
        .flat_map(|line| [line.start.x, line.end.x])
        .max()
        .unwrap();

    let max_y = lines
        .iter()
        .flat_map(|line| [line.start.y, line.end.y])
        .max()
        .unwrap();

    let mut grid = vec![0; (max_x + 1) * (max_y + 1)];

    for (i, intersections) in grid.iter_mut().enumerate() {
        let p = Point {
            x: i % max_x,
            y: i / max_x,
        };
        for line in lines.iter() {
            if line.intersects(&p) {
                *intersections += 1;
            }
        }
    }

    grid.iter().filter(|i| **i >= 2).count()
}

#[aoc(day5 part1)]
pub fn solve_part1(lines: &[Line]) -> usize {
    calculate_intersection(lines.to_vec(), |line| {
        line.is_horizontal() || line.is_vertical()
    })
}

#[aoc(day5 part2)]
pub fn solve_part2(lines: &[Line]) -> usize {
    calculate_intersection(lines.to_vec(), |_| true)
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0,9 -> 5,9\n8,0 -> 0,8\n9,4 -> 3,4\n2,2 -> 2,1\n7,0 -> 7,4\n6,4 -> 2,0\n0,9 -> 2,9\n3,4 -> 1,4\n0,0 -> 8,8\n5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(input_generator(INPUT).unwrap().as_slice()), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(input_generator(INPUT).unwrap().as_slice()), 12)
    }
}
