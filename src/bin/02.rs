use itertools::Itertools;
use std::cmp::max;
use std::collections::HashMap;

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

fn split(s: &str) -> (&str, &str) {
    let m = s.len() / 2;
    (&s[..m], &s[m..])
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

fn solve_for_length(start: &str, end: &str, length: usize) -> HashMap<usize, u64> {
    let (ls, le) = (start.len(), end.len());
    let (ms, me) = (ls / length, le / length);

    let mut answers: HashMap<usize, u64> = HashMap::new();

    if ls == le {
        if ls % length != 0 || ls == 1 {
            return answers;
        }

        let (sp, ep) = (&start[..length], &end[..length]);
        let (spu, epu) = (parse(sp), parse(ep));
        let (sc, ec) = (sp.repeat(ms), ep.repeat(me));
        let (sc, ec) = (sc.as_str(), ec.as_str());
        let (scu, ecu) = (parse(sc), parse(ec));

        if sp == ep {
            if sc >= start && ec <= end {
                (*answers.entry(ms).or_insert(0)) += scu;
            }
        } else {
            if sc >= start {
                (*answers.entry(ms).or_insert(0)) += scu;
            }
            if ec <= end {
                (*answers.entry(ms).or_insert(0)) += ecu;
            }

            let sum = sum_between(spu + 1, epu - 1);
            let mult = gen_mult(length, ms);
            (*answers.entry(ms).or_insert(0)) += sum * mult;
        }
    } else {
        if ls % length == 0 && ls > length {
            let sp = &start[..length];
            let spu = parse(sp);
            let sc = sp.repeat(ms);
            let sc = sc.as_str();
            let scu = parse(sc);

            if sc >= start {
                (*answers.entry(ms).or_insert(0)) += scu;
            }

            let e = pow10(length) - 1;

            let sum = sum_between(spu + 1, e);
            let mult = gen_mult(length, ms);
            (*answers.entry(ms).or_insert(0)) += sum * mult;
        }

        if le % length == 0 {
            let ep = &end[..length];
            let epu = parse(ep);
            let ec = ep.repeat(me);
            let ec = ec.as_str();
            let ecu = parse(ec);

            if ec <= end {
                (*answers.entry(me).or_insert(0)) += ecu;
            }

            let s = pow10(length - 1);

            let sum = sum_between(s, epu - 1);
            let mult = gen_mult(length, me);
            (*answers.entry(me).or_insert(0)) += sum * mult;
        }

        (max(ms + 1, 2)..=(me - 1)).for_each(|m| {
            let s = pow10(length - 1);
            let e = pow10(length) - 1;
            let sum = sum_between(s, e);
            let mult = gen_mult(length, m);
            *answers.entry(m).or_insert(0) += sum * mult;
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
            let mut possibilities = 0u64;
            let (ls, le) = (start.len(), end.len());
            let (s_first, s_second) = split(start);
            let (e_first, e_second) = split(end);
            if ls == le {
                if ls % 2 == 0 {
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
                        let sum = sum_between(s + 1, e - 1);
                        let sum2 = sum + sum * pow10(ls / 2);
                        possibilities += sum2;
                    }
                }
            } else {
                if ls % 2 == 0 {
                    let s = parse(s_first);
                    if s_second <= s_first {
                        possibilities += value(s_first);
                    }

                    let e = pow10(ls / 2);
                    let sum = sum_between(s + 1, e - 1);
                    let sum2 = sum + sum * e;
                    possibilities += sum2;
                }

                if le % 2 == 0 {
                    let e = parse(e_first);
                    if e_second >= e_first {
                        possibilities += value(e_first);
                    }

                    let s = pow10(le / 2 - 1);
                    let sum = sum_between(s, e - 1);
                    let sum2 = sum + sum * s * 10;
                    possibilities += sum2;
                }

                let (a, b) = (ls / 2 + 1, (le - 1) / 2);
                let betweens = (a..=b)
                    .map(|digits| {
                        let s = pow10(digits - 1);
                        let e = 10 * s;
                        let sum = sum_between(s, e - 1);
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
    let res: u64 = input
        .trim()
        .split(',')
        .map(|range| range.split('-').collect_tuple::<(&str, &str)>().unwrap())
        .map(|(start, end)| {
            let mut result = 0;
            let mut res_map: HashMap<(usize, usize), u64> = HashMap::new();

            (1..=(end.len() / 2)).for_each(|length| {
                let mut possibilities = solve_for_length(start, end, length);

                possibilities.iter_mut().for_each(|(m, v)| {
                    res_map.iter().for_each(|(&(m2, l2), &v2)| {
                        if m * length == m2 * l2 && length % l2 == 0 {
                            *v -= v2;
                        }
                    });
                    res_map.insert((*m, length), *v);
                    result += *v;
                });
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
