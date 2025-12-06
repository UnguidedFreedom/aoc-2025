use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut inputs = input
        .trim()
        .lines()
        .map(|l| l.split(' ').filter(|s| !s.is_empty()).collect_vec())
        .collect_vec();

    let operations = inputs.pop().unwrap();

    let nums = inputs
        .into_iter()
        .map(|v| {
            v.into_iter()
                .map(|s| s.parse::<u64>().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let res = operations
        .into_iter()
        .enumerate()
        .map(|(i, op)| {
            let numbers = nums.iter().map(|v| v[i]);
            match op {
                "+" => numbers.sum::<u64>(),
                "*" => numbers.product(),
                _ => panic!(),
            }
        })
        .sum::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut inputs = input.lines().map(|l| l.chars().collect_vec()).collect_vec();

    let operations = inputs.pop().unwrap();

    let mut nums: Vec<u64> = vec![];
    let mut skip = false;

    let res = operations
        .into_iter()
        .enumerate()
        .rev()
        .map(|(i, c)| {
            if skip {
                skip = false;
                return 0;
            }

            let num: String = inputs.iter().map(|v| v[i]).filter(|&c| c != ' ').collect();
            nums.push(num.parse::<u64>().unwrap());

            match c {
                '+' => {
                    let r = nums.iter().sum::<u64>();
                    nums = vec![];
                    skip = true;
                    r
                }
                '*' => {
                    let r = nums.iter().product::<u64>();
                    nums = vec![];
                    skip = true;
                    r
                }
                _ => 0,
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
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
