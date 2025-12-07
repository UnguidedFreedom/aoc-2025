use itertools::Itertools;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|l| l.chars());

    let start = lines.next().unwrap().position(|c| c == 'S').unwrap();

    let mut res = 0;

    lines.fold(HashSet::from([start]), |beams, line| {
        let line = line.collect_vec();
        let mut new_beams = HashSet::with_capacity(2 * beams.len());
        beams.into_iter().for_each(|beam| match line[beam] {
            '^' => {
                res += 1;
                new_beams.insert(beam - 1);
                new_beams.insert(beam + 1);
            }
            _ => {
                new_beams.insert(beam);
            }
        });
        new_beams
    });

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines().map(|l| l.chars());

    let start = lines.next().unwrap().position(|c| c == 'S').unwrap();

    let out = lines.fold(HashMap::from([(start, 1)]), |beams, line| {
        let line = line.collect_vec();
        let mut new_beams = HashMap::with_capacity(2 * beams.len());
        beams
            .into_iter()
            .for_each(|(beam, count)| match line[beam] {
                '^' => {
                    *new_beams.entry(beam - 1).or_default() += count;
                    *new_beams.entry(beam + 1).or_default() += count;
                }
                _ => {
                    *new_beams.entry(beam).or_default() += count;
                }
            });
        new_beams
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
