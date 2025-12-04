use itertools::Itertools;

advent_of_code::solution!(4);

fn get_adjacent(grid: &[Vec<char>], i: isize, j: isize) -> char {
    if (0..grid.len() as isize).contains(&i) {
        let row = &grid[i as usize];
        if (0..row.len() as isize).contains(&j) {
            return row[j as usize];
        }
    }
    '.'
}

fn count_adjacents(grid: &[Vec<char>], i: isize, j: isize) -> u64 {
    (-1isize..=1)
        .map(|di| {
            (-1isize..=1)
                .filter(|&dj| {
                    if di != 0 || dj != 0 {
                        get_adjacent(grid, i + di, j + dj) == '@'
                    } else {
                        false
                    }
                })
                .count() as u64
        })
        .sum::<u64>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let res = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, &c)| c == '@' && count_adjacents(&grid, i as isize, j as isize) < 4)
                .count() as u64
        })
        .sum::<u64>();

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
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
