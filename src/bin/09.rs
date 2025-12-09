use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BTreeMap;

advent_of_code::solution!(9);

fn sorted_pair<T: PartialOrd>(a: T, b: T) -> (T, T) {
    if a < b { (a, b) } else { (b, a) }
}

fn contained(map: &BTreeMap<u64, Vec<(u64, u64)>>, a1: u64, a2: u64, b1: u64, b2: u64) -> bool {
    map.range(a1 + 1..a2)
        .all(|(_, v)| v.iter().all(|&(start, end)| end <= b1 || start >= b2))
}

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .tuple_combinations()
        .map(|(a, b)| (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1))
        .max()
        .unwrap();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let corners = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap()
        })
        .collect_vec();

    let mut horiz: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();
    let mut vert: BTreeMap<u64, Vec<(u64, u64)>> = BTreeMap::new();

    corners.iter().circular_tuple_windows().for_each(|(a, b)| {
        if a.0 == b.0 {
            horiz.entry(a.0).or_default().push(sorted_pair(a.1, b.1));
        } else {
            vert.entry(a.1).or_default().push(sorted_pair(a.0, b.0));
        }
    });

    let res = corners
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1)))
        .sorted_by_key(|(_, _, d)| Reverse(*d))
        .find(|(a, b, _)| {
            let (i1, i2) = sorted_pair(a.0, b.0);
            let (j1, j2) = sorted_pair(a.1, b.1);

            contained(&horiz, i1, i2, j1, j2) && contained(&vert, j1, j2, i1, i2)
        })
        .unwrap()
        .2;

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
