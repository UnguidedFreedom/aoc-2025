use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(2);

fn parse(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn pow10(p: usize) -> u64 {
    10u64.pow(p as u32)
}

fn sum_between(start: u64, end: u64) -> u64 {
    if end < start {
        0
    } else {
        (end * (end + 1) - start * (start - 1)) / 2
    }
}

fn gen_mult(length: usize, parts: usize) -> u64 {
    parse(
        format!(
            "{}1",
            format!("1{}", "0".repeat(length - 1)).repeat(parts - 1)
        )
        .as_str(),
    )
}

fn solve_for_parts(start: &str, end: &str, parts: usize) -> HashMap<usize, u64> {
    let (ls, le) = (start.len(), end.len());
    let (su, eu) = (parse(start), parse(end));

    let mut answers: HashMap<usize, u64> = HashMap::new();

    if ls == le {
        if ls % parts != 0 {
            return answers;
        }
        let length = ls / parts;
        let mult = gen_mult(length, parts);

        let (sp, ep) = (&start[..length], &end[..length]);
        let (spu, epu) = (parse(sp), parse(ep));
        let (scu, ecu) = (spu*mult, epu*mult);

        if spu == epu {
            if scu >= su && ecu <= eu {
                (*answers.entry(length).or_insert(0)) += scu;
            }
        } else {
            if scu >= su {
                (*answers.entry(length).or_insert(0)) += scu;
            }
            if ecu <= eu {
                (*answers.entry(length).or_insert(0)) += ecu;
            }

            let sum = sum_between(spu + 1, epu - 1);
            (*answers.entry(length).or_insert(0)) += sum * mult;
        }
    } else {
        if ls % parts == 0 {
            let length = ls / parts;
            let mult = gen_mult(length, parts);

            let sp = &start[..length];
            let spu = parse(sp);
            let scu = spu*mult;

            if scu >= su {
                (*answers.entry(length).or_insert(0)) += scu;
            }

            let e = pow10(length) - 1;

            let sum = sum_between(spu + 1, e);
            (*answers.entry(length).or_insert(0)) += sum * mult;
        }

        if le % parts == 0 {
            let length = le / parts;
            let mult = gen_mult(length, parts);

            let ep = &end[..length];
            let epu = parse(ep);
            let ecu = epu*mult;

            if ecu <= eu {
                (*answers.entry(length).or_insert(0)) += ecu;
            }

            let s = pow10(length - 1);

            let sum = sum_between(s, epu - 1);
            (*answers.entry(length).or_insert(0)) += sum * mult;
        }

        ((ls / parts) + 1..=((le-1) / parts)).for_each(|length| {
            let s = pow10(length -1);
            let e = pow10(length)-1;
            let sum = sum_between(s, e);
            let mult = gen_mult(length, parts);
            (*answers.entry(length).or_insert(0)) += sum * mult;
        });
    }

    answers
}

pub fn part_one(input: &str) -> Option<u64> {
    let res: u64 = input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(start, end)| {
            let answers = solve_for_parts(start, end, 2);
            answers.values().sum::<u64>()
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res: u64 = input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(start, end)| {
            let mut result = 0;

            let mut res_map: HashMap<usize, HashMap<usize, u64>> = HashMap::new();
            (2..=end.len()).for_each(|parts| {
                let possibilities = solve_for_parts(start, end, parts);
                for (l, val) in possibilities {
                    res_map
                        .entry(l)
                        .or_insert(HashMap::new())
                        .insert(parts, val);
                }
            });

            let sorted_keys = res_map.keys().copied().sorted().collect_vec();

            sorted_keys.iter().for_each(|l| {
                sorted_keys
                    .iter()
                    .take_while(|&l2| l2 < l)
                    .filter(|&l2| l % l2 == 0)
                    .for_each(|l2| {
                        let map2 = res_map.get(l2).cloned().unwrap();
                        let map = res_map.get_mut(l).unwrap();
                        map.iter_mut().for_each(|(k, v)| {
                            *v -= map2.get(&(l * k / l2)).unwrap_or(&0);
                        })
                    });
                result += res_map[l].values().sum::<u64>();
            });
            result
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
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
