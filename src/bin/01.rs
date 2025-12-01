advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|s| {
            let value = s[1..].parse::<i64>().unwrap();
            match s.starts_with("L") {
                true => -value,
                false => value,
            }
        })
        .fold((50i64, 0u64), |(val, count), value| {
            let new_val = (val + value).rem_euclid(100);
            (new_val, count + if new_val == 0 { 1 } else { 0 })
        })
        .1;

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|s| {
            let value = s[1..].parse::<i64>().unwrap();
            match s.starts_with("L") {
                true => -value,
                false => value,
            }
        })
        .fold((50i64, 0u64), |(val, count), value| {
            let new_val = val + value;
            let mut new_count = count;
            if new_val >= 100 {
                new_count += (new_val / 100) as u64;
            } else if new_val <= 0 {
                new_count += ((-new_val) / 100) as u64;
                if val != 0 {
                    new_count += 1;
                }
            }
            (new_val.rem_euclid(100), new_count)
        })
        .1;

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
        assert_eq!(result, Some(6));
    }
}
