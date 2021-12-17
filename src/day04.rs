use super::*;

const BOARD_SIZE: usize = 5;

#[aoc(day4, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let mut lines = input.lines();

    // parse "random" numbers
    let mut nums = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| u32::from_str_radix(c, 10).unwrap());

    // parse all the boards
    let mut boards: Vec<_> = lines
        .map(|line| line.split_whitespace())
        .flatten()
        .map(|n| Some(u32::from_str_radix(n, 10).unwrap()))
        .collect::<Vec<_>>()
        .chunks_exact(25)
        .map(|s| s.to_vec())
        .collect();

    // check wether a given board fulfills winning condition (marked row or col)
    let has_won = |board: &[Option<u32>]| {
        (0..BOARD_SIZE).any(|i| {
            board[BOARD_SIZE * i..BOARD_SIZE * (i + 1)]
                .iter()
                .all(Option::is_none)
                || board[i..=(BOARD_SIZE - 1) * BOARD_SIZE + i]
                    .iter()
                    .step_by(BOARD_SIZE)
                    .all(Option::is_none)
        })
    };

    // loop through the numbers
    let (board, num) = 'outer: loop {
        let num = nums.next().unwrap();

        // mark the number on each board if it exists and check if the board has won
        for board in boards.iter_mut() {
            match board.iter().position(|&n| n == Some(num)) {
                Some(i) => {
                    board[i] = None;
                    if has_won(board) {
                        break 'outer (board, num);
                    }
                }
                None => {
                    continue;
                }
            }
        }
    };

    // sum all unmarked numbers on the winning board and multiply by last number
    board.iter().fold(0, |sum, n| match n {
        Some(n) => sum + n,
        None => sum,
    }) * num
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let mut lines = input.lines();

    // parse "random" numbers
    let mut nums = lines
        .next()
        .unwrap()
        .split(',')
        .map(|c| u32::from_str_radix(c, 10).unwrap());

    // parse all the boards
    let mut boards: Vec<_> = lines
        .map(|line| line.split_whitespace())
        .flatten()
        .map(|n| Some(u32::from_str_radix(n, 10).unwrap()))
        .collect::<Vec<_>>()
        .chunks_exact(25)
        .map(|s| s.to_vec())
        .collect();

    // check wether a given board fulfills winning condition (marked row or col)
    let has_won = |board: &[Option<u32>]| {
        (0..BOARD_SIZE).any(|i| {
            board[BOARD_SIZE * i..BOARD_SIZE * (i + 1)]
                .iter()
                .all(Option::is_none)
                || board[i..=(BOARD_SIZE - 1) * BOARD_SIZE + i]
                    .iter()
                    .step_by(BOARD_SIZE)
                    .all(Option::is_none)
        })
    };

    let mut board_idx = 0;
    let mut num_idx = 0;

    'outer: for (j, board) in boards.iter_mut().enumerate() {
        for (i, num) in nums.clone().enumerate() {
            match board.iter().position(|&n| n == Some(num)) {
                Some(p) => {
                    board[p] = None;
                    if has_won(board) {
                        if i > num_idx {
                            num_idx = i;
                            board_idx = j;
                        }
                        continue 'outer;
                    }
                }
                None => {
                    continue;
                }
            }
        }
    }

    // sum all unmarked numbers on the winning board and multiply by last number
    boards[board_idx].iter().fold(0, |sum, n| match n {
        Some(n) => sum + n,
        None => sum,
    }) * nums.nth(num_idx).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"),
            4512
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2("7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1\n\n22 13 17 11  0\n 8  2 23  4 24\n21  9 14 16  7\n 6 10  3 18  5\n 1 12 20 15 19\n\n 3 15  0  2 22\n 9 18 13 17  5\n19  8  7 25 23\n20 11 10 24  4\n14 21 16 12  6\n\n14 21 17 24  4\n10 16 15  9 19\n18  8 23 26 20\n22 11 13  6  5\n 2  0 12  3  7"),
            1924
        )
    }
}
