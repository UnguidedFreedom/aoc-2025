use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(12);

fn generate(row_size: u64, data: [u64; 9]) -> u64 {
    (data[0] | data[1] << 1 | data[2] << 2)
        | (data[3] | data[4] << 1 | data[5] << 2) << row_size
        | (data[6] | data[7] << 1 | data[8] << 2) << (2 * row_size)
}

fn solve(
    dimens: (u64, u64),
    presents: &[[u64; 8]],
    tree: u64,
    counts: u64,
    idx: u64,
    cache: &mut HashMap<(u64, u64), bool>,
) -> bool {
    if counts == 0 {
        return true;
    }
    if let Some(res) = cache.get(&(tree, counts)) {
        return *res;
    }

    let mut idx = idx;
    while counts & (0xff << (idx * 8)) == 0 {
        idx += 1;
    }

    let new_counts = counts - (1 << (idx * 8));
    let possibilities = &presents[idx as usize];

    for i in 0..dimens.0 - 2 {
        for j in 0..dimens.1 - 2 {
            let offset = i * dimens.1 + j;
            let local_res = possibilities
                .iter()
                .map(|&p| p << offset)
                .filter(|&p| tree & p == 0)
                .any(|p| solve(dimens, presents, tree | p, new_counts, idx, cache));
            if local_res {
                cache.insert((tree, counts), local_res);
                return true;
            }
        }
    }

    cache.insert((tree, counts), false);
    false
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut chunks = input.split("\n\n").collect_vec();
    let cases = chunks
        .pop()
        .unwrap()
        .lines()
        .map(|line| {
            let (first, second) = line.split_once(": ").unwrap();
            let dimensions = first
                .split('x')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple::<(u64, u64)>()
                .unwrap();
            let pres_num = second
                .split(' ')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec();
            (dimensions, pres_num)
        })
        .collect_vec();

    let presents = chunks
        .into_iter()
        .map(|s| {
            s.lines()
                .skip(1)
                .map(|s| s.chars().map(|c| (c == '#') as u64).collect_vec())
                .collect_vec()
        })
        .collect_vec();

    let presents = presents
        .into_iter()
        .map(|v| (v.iter().flatten().filter(|b| **b == 1).count() as u64, v))
        .collect_vec();

    let res = cases
        .into_iter()
        .filter(|(dimens, num_presents)| {
            let presents_size = num_presents
                .iter()
                .enumerate()
                .map(|(i, &n)| n * presents[i].0)
                .sum::<u64>();
            let area = dimens.0 * dimens.1;
            let pres_count = num_presents.iter().cloned().sum::<u64>();
            let fit_presents = (dimens.0 / 3) * (dimens.1 / 3);

            if presents_size > area {
                false
            } else if pres_count <= fit_presents {
                true
            } else {
                let local_presents = presents
                    .iter()
                    .map(|(_, v)| {
                        [
                            generate(
                                // 0d, not flipped
                                dimens.1,
                                [
                                    v[0][0], v[0][1], v[0][2], v[1][0], v[1][1], v[1][2], v[2][0],
                                    v[2][1], v[2][2],
                                ],
                            ),
                            generate(
                                // 0d, flipped
                                dimens.1,
                                [
                                    v[0][2], v[0][1], v[0][0], v[1][2], v[1][1], v[1][0], v[2][2],
                                    v[2][1], v[2][0],
                                ],
                            ),
                            generate(
                                // 90d, not flipped
                                dimens.1,
                                [
                                    v[0][2], v[1][2], v[2][2], v[0][1], v[1][1], v[2][1], v[0][0],
                                    v[1][0], v[2][0],
                                ],
                            ),
                            generate(
                                // 90d, flipped
                                dimens.1,
                                [
                                    v[2][2], v[1][2], v[0][2], v[2][1], v[1][1], v[0][1], v[2][0],
                                    v[1][0], v[0][0],
                                ],
                            ),
                            generate(
                                // 180d, not flipped
                                dimens.1,
                                [
                                    v[2][2], v[2][1], v[2][0], v[1][2], v[1][1], v[1][0], v[0][2],
                                    v[0][1], v[0][0],
                                ],
                            ),
                            generate(
                                // 180d, flipped
                                dimens.1,
                                [
                                    v[2][0], v[2][1], v[2][2], v[1][0], v[1][1], v[1][2], v[0][0],
                                    v[0][1], v[0][2],
                                ],
                            ),
                            generate(
                                // 270d, not flipped
                                dimens.1,
                                [
                                    v[2][0], v[1][0], v[0][0], v[2][1], v[1][1], v[0][1], v[2][2],
                                    v[1][2], v[0][2],
                                ],
                            ),
                            generate(
                                // 270d, flipped
                                dimens.1,
                                [
                                    v[0][0], v[1][0], v[2][0], v[0][1], v[1][1], v[2][1], v[0][2],
                                    v[1][2], v[2][2],
                                ],
                            ),
                        ]
                    })
                    .collect_vec();

                let mut cache = HashMap::new();

                let counts = num_presents
                    .iter()
                    .enumerate()
                    .fold(0, |acc, (i, n)| acc | (*n << (8 * i)));

                solve(*dimens, &local_presents, 0, counts, 0, &mut cache)
            }
        })
        .count() as u64;

    Some(res)
}

pub fn part_two(_input: &str) -> Option<u64> {
    Some(23)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
