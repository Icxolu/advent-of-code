use super::*;
use std::collections::HashMap;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let map = input.lines().fold(HashMap::new(), |mut map, num| {
        for (i, c) in num.chars().rev().enumerate() {
            match c {
                '0' => map.entry(i).or_insert((0, 0)).0 += 1,
                '1' => map.entry(i).or_insert((0, 0)).1 += 1,
                _ => panic!("unexpected input"),
            }
        }
        map
    });

    let (gamma, epsilon) = map
        .into_iter()
        .map(|(i, (zero, one))| (i, zero < one))
        .fold((0_u32, 0_u32), |(gamma, eps), (i, n)| {
            if n {
                (gamma + 2_u32.pow(i as u32), eps)
            } else {
                (gamma, eps + 2_u32.pow(i as u32))
            }
        });

    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let nums: Vec<_> = input.lines().collect();

    let most_common_bit = |nums: &[&str], i| {
        let mut count = 0;
        for num in nums {
            if num.chars().nth(i).unwrap() == '0' {
                count -= 1;
            } else {
                count += 1;
            }
        }
        if count >= 0 {
            '1'
        } else {
            '0'
        }
    };

    let least_common_bit = |nums: &[&str], i| match most_common_bit(nums, i) {
        '0' => '1',
        '1' => '0',
        _ => unreachable!(),
    };

    let (mut o2, mut co2) = (nums.clone(), nums.clone());
    let mut bit = 0;

    while o2.len() > 1 {
        let mcb = most_common_bit(&o2.clone(), bit);
        o2.retain(|num| num.chars().nth(bit).unwrap() == mcb);
        bit += 1;
    }

    bit = 0;
    while co2.len() > 1 {
        let lcb = least_common_bit(&co2.clone(), bit);
        co2.retain(|num| num.chars().nth(bit).unwrap() == lcb);
        bit += 1;
    }

    u32::from_str_radix(o2[0], 2).unwrap() * u32::from_str_radix(co2[0], 2).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            solve_part1("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"),
            198
        )
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            solve_part2("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010"),
            230
        )
    }
}
