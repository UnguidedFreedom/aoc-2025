use itertools::Itertools;

advent_of_code::solution!(2);

fn value(half: &str) -> u64 {
    parse((half.to_string() + half).as_str())
}

fn parse(s: &str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn pow10(p: usize) -> u64 {
    10u64.pow(p as u32)
}

pub fn part_one(input: &str) -> Option<u64> {
    let res: u64 = input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(start, end)| {
            let mut possibilities = 0u64;
            let (ls, le) = (start.len(), end.len());
            if ls == le {
                if ls % 2 == 0 {
                    let (s_first, s_second) = (&start[..ls / 2], &start[ls / 2..]);
                    let (e_first, e_second) = (&end[..le / 2], &end[le / 2..]);

                    if s_first == e_first {
                        if s_second <= s_first && e_second >= s_first {
                            possibilities += value(s_first);
                        }
                    } else {
                        if s_second <= s_first {
                            possibilities += value(s_first);
                        }

                        if e_second >= e_first {
                            possibilities += value(e_first);
                        }

                        let (s, e) = (parse(s_first), parse(e_first));
                        let sum = e * (e - 1) / 2 - s * (s + 1) / 2;
                        let sum2 = sum + sum * pow10(ls / 2);
                        possibilities += sum2;
                    }
                }
            } else {
                if ls % 2 == 0 {
                    let (s_first, s_second) = (&start[..ls / 2], &start[ls / 2..]);
                    let s = parse(s_first);
                    if s_second <= s_first {
                        possibilities += value(s_first);
                    }

                    let e = pow10(ls / 2);
                    let sum = e * (e - 1) / 2 - s * (s + 1) / 2;
                    let sum2 = sum + sum * e;
                    possibilities += sum2;
                }

                if le % 2 == 0 {
                    let (e_first, e_second) = (&end[..le / 2], &end[le / 2..]);
                    let e = parse(e_first);
                    if e_second >= e_first {
                        possibilities += value(e_first);
                    }

                    let s = pow10(le / 2 - 1);
                    let sum = e * (e - 1) / 2 - s * (s - 1) / 2;
                    let sum2 = sum + sum * s * 10;
                    possibilities += sum2;
                }

                let (a, b) = (ls / 2 + 1, (le - 1) / 2);
                let betweens = (a..=b)
                    .map(|digits| {
                        let s = pow10(digits - 1);
                        let e = 10 * s;
                        let sum = e * (e - 1) / 2 - s * (s - 1) / 2;
                        sum + e * sum
                    })
                    .sum::<u64>();
                possibilities += betweens;
            }
            possibilities
        })
        .sum();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }
}
