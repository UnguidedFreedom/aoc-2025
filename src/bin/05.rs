use itertools::Itertools;
use std::cmp::max;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let mut iter = input.trim().lines();

    let ranges = (&mut iter)
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (a, b) = l
                .split('-')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap();
            a..=b
        })
        .collect_vec();

    let res = iter
        .filter(|s| {
            let n = s.parse::<u64>().unwrap();
            ranges.iter().any(|r| r.contains(&n))
        })
        .count() as u64;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges: Vec<(u64, u64)> = input
        .trim()
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split('-')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .sorted_by(|a: &(u64, u64), b: &(u64, u64)| Ord::cmp(&a.0, &b.0))
        .collect_vec();

    let mut max_end: u64 = 0;

    let res = ranges
        .iter()
        .enumerate()
        .map(|(i, range)| {
            max_end = max(max_end, range.1);
            let i2 = i + 1;
            if i2 == ranges.len() {
                max_end - range.0 + 1
            } else {
                let range2 = &ranges[i2];
                if range2.0 <= max_end {
                    range2.0 - range.0
                } else {
                    max_end - range.0 + 1
                }
            }
        })
        .sum::<u64>();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
