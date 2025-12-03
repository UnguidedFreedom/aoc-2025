advent_of_code::solution!(3);

fn highest(input: &str) -> (usize, char) {
    input
        .char_indices()
        .reduce(|(pos_max, char_max), (pos, char)| {
            if char > char_max {
                (pos, char)
            } else {
                (pos_max, char_max)
            }
        })
        .unwrap()
}

const ZERO: u64 = '0' as u64;

fn solve_n(bank: &str, n: usize) -> u64 {
    let l = bank.len();
    let mut start: usize = 0;
    let mut res = 0;
    (0..n).rev().for_each(|k| {
        let (pos, char) = highest(&bank[start..l - k]);
        res = res * 10 + char as u64 - ZERO;
        start += pos + 1;
    });
    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input.lines().map(|bank| solve_n(bank, 2)).sum::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input.lines().map(|bank| solve_n(bank, 12)).sum::<u64>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
