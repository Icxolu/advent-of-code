use super::*;

#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|l| {
            let mut stack = Vec::new();

            for c in l.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        if Some(c) != stack.pop() {
                            return Some(match c {
                                ')' => 3,
                                ']' => 57,
                                '}' => 1197,
                                '>' => 25137,
                                _ => unreachable!(),
                            });
                        }
                    }
                }
            }
            None
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let mut scores: Vec<_> = input
        .lines()
        .filter_map(|l| {
            let mut stack = Vec::new();

            for c in l.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    _ => {
                        if Some(c) != stack.pop() {
                            return None;
                        }
                    }
                }
            }

            let score = stack.into_iter().rev().fold(0, |score, c| {
                5 * score
                    + match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
            });

            Some(score)
        })
        .collect();

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "[({(<(())[]>[[{[]{<()<>>\n[(()[<>])]({[<{<<[]>>(\n{([(<{}[<>[]}>{[]{[(<()>\n(((({<>}<{<{<>}{[]{[]{}\n[[<[([]))<([[{}[[()]]]\n[{[{({}]{}}([{[{{{}}([]\n{<[[]]>}<{[{[{[]{()[[[]\n[<(<(<(<{}))><([]([]()\n<{([([[(<>()){}]>(<<{{\n<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 26397)
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 288957)
    }
}
