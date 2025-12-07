use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let start = lines
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    let mut res = 0;

    lines.fold(HashSet::from([start]), |beams, line| {
        beams
            .into_iter()
            .flat_map(|beam| match line.chars().nth(beam).unwrap() {
                '^' => {
                    res += 1;
                    vec![beam - 1, beam + 1]
                }
                _ => vec![beam],
            })
            .collect()
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let start = lines
        .next()
        .unwrap()
        .chars()
        .position(|c| c == 'S')
        .unwrap();

    let out = lines.fold(HashMap::from([(start, 1)]), |beams, line| {
        beams
            .into_iter()
            .flat_map(|(beam, count)| match line.chars().nth(beam).unwrap() {
                '^' => {
                    vec![(beam - 1, count), (beam + 1, count)]
                }
                _ => vec![(beam, count)],
            })
            .into_grouping_map()
            .sum()
    });

    let res = out.values().sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
