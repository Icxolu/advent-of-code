use super::*;

#[derive(Debug, Clone)]
pub struct Lanternfish {
    timer: usize,
}

fn simulate_lanternfish(fish: Vec<Lanternfish>, days: usize) -> usize {
    let mut fish_counts = vec![0; 9];

    for f in fish {
        fish_counts[f.timer] += 1;
    }

    for _ in 0..days {
        fish_counts.rotate_left(1);
        fish_counts[6] += fish_counts[8]
    }

    fish_counts.iter().sum()
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Lanternfish> {
    input
        .split(',')
        .map(|timer| Lanternfish {
            timer: timer.parse().unwrap(),
        })
        .collect()
}

#[aoc(day6 part1)]
pub fn solve_part1(fish: &[Lanternfish]) -> usize {
    simulate_lanternfish(fish.to_vec(), 80)
}

#[aoc(day6 part2)]
pub fn solve_part2(fish: &[Lanternfish]) -> usize {
    simulate_lanternfish(fish.to_vec(), 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation() {
        let fish = input_generator("3,4,3,1,2");
        assert_eq!(simulate_lanternfish(fish, 18), 26)
    }

    #[test]
    fn test_part1() {
        let fish = input_generator("3,4,3,1,2");
        assert_eq!(solve_part1(&fish), 5934)
    }

    #[test]
    fn test_part2() {
        let fish = input_generator("3,4,3,1,2");
        assert_eq!(solve_part2(&fish), 26984457539)
    }
}
