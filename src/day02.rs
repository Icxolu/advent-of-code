use super::*;

pub enum Command {
    Horizontal(i32),
    Vertical(i32),
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Command>, nom::Err<()>> {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{char, i32, line_ending},
        combinator::map,
        multi::separated_list1,
        sequence::separated_pair,
    };

    let forward = tag("forward");
    let down = tag("down");
    let up = tag("up");

    let cmd = map(
        separated_pair(alt((forward, down, up)), char(' '), i32),
        |(kind, val)| match kind {
            "forward" => Command::Horizontal(val),
            "down" => Command::Vertical(val),
            "up" => Command::Vertical(-val),
            _ => unreachable!(),
        },
    );
    let mut parse = separated_list1(line_ending, cmd);

    parse(input).map(|(_, p)| p)
}

#[aoc(day2, part1)]
pub fn solve_part1(cmds: &[Command]) -> i32 {
    let position = cmds.iter().fold((0, 0), |(x, y), cmd| match cmd {
        Command::Horizontal(d) => (x + d, y),
        Command::Vertical(d) => (x, y + d),
    });

    position.0 * position.1
}

#[aoc(day2, part2)]
pub fn solve_part2(cmds: &[Command]) -> i32 {
    let position = cmds.iter().fold((0, 0, 0), |(x, y, aim), cmd| match cmd {
        Command::Horizontal(d) => (x + d, y + aim * d, aim),
        Command::Vertical(d) => (x, y, aim + d),
    });

    position.0 * position.1
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(input_generator(INPUT).unwrap().as_slice()), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(input_generator(INPUT).unwrap().as_slice()), 900)
    }
}
