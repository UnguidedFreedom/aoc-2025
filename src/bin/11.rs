use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;

advent_of_code::solution!(11);

#[derive(Default, Copy, Clone, Debug)]
struct Partial {
    pub none: u64,
    pub fft: u64,
    pub dac: u64,
    pub both: u64,
}

impl Partial {
    pub fn new_unit() -> Self {
        Partial {
            none: 1,
            fft: 0,
            dac: 0,
            both: 0,
        }
    }

    pub fn fft(&self) -> Self {
        Partial {
            none: 0,
            fft: self.fft + self.none,
            dac: 0,
            both: self.both + self.dac,
        }
    }

    pub fn dac(&self) -> Self {
        Partial {
            none: 0,
            fft: 0,
            dac: self.dac + self.none,
            both: self.both + self.fft,
        }
    }

    pub fn total(&self) -> u64 {
        self.none + self.fft + self.dac + self.both
    }
}

impl Add for Partial {
    type Output = Partial;

    fn add(self, rhs: Self) -> Self::Output {
        Partial {
            none: self.none + rhs.none,
            fft: self.fft + rhs.fft,
            dac: self.dac + rhs.dac,
            both: self.both + rhs.both,
        }
    }
}

fn solve<'a>(
    circuits: &HashMap<&str, Vec<&'a str>>,
    cache: &mut HashMap<&'a str, Partial>,
    start: &'a str,
) -> Partial {
    if start == "out" {
        return Partial::new_unit();
    } else if cache.contains_key(&start) {
        return *cache.get(&start).unwrap();
    }

    let res = circuits[&start]
        .iter()
        .fold(Partial::default(), |acc, s| acc + solve(circuits, cache, s));

    let res = match start {
        "fft" => res.fft(),
        "dac" => res.dac(),
        _ => res,
    };

    cache.insert(start, res);

    res
}

pub fn part_one(input: &str) -> Option<u64> {
    let circuits: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let input = parts.next().unwrap().trim_end_matches(':');
            (input, parts.collect_vec())
        })
        .collect();

    let mut cache = HashMap::new();
    let res = solve(&circuits, &mut cache, "you").total();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let circuits: HashMap<&str, Vec<&str>> = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ');
            let input = parts.next().unwrap().trim_end_matches(':');
            (input, parts.collect_vec())
        })
        .collect();

    let mut cache = HashMap::new();
    let res = solve(&circuits, &mut cache, "svr").both;

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
