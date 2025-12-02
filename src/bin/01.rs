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
        .fold((50i64, 0u64), |(dial, count), delta| {
            let new_dial = (dial + delta).rem_euclid(100);
            (new_dial, if new_dial == 0 { count + 1 } else { count })
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
        .fold((50i64, 0u64), |(dial, mut count), delta| {
            let raw_new_dial = dial + delta;
            if raw_new_dial >= 100 {
                count += (raw_new_dial / 100) as u64;
            } else if raw_new_dial <= 0 {
                count += ((-raw_new_dial) / 100) as u64;
                if dial != 0 {
                    count += 1;
                }
            }
            (raw_new_dial.rem_euclid(100), count)
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
