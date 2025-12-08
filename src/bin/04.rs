use itertools::Itertools;

advent_of_code::solution!(4);

const ADJACENTS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const COMPARE: usize = 4;

fn get_adjacent(grid: &[Vec<bool>], i: isize, j: isize) -> bool {
    if (0..grid.len() as isize).contains(&i) {
        let row = &grid[i as usize];
        if (0..row.len() as isize).contains(&j) {
            return row[j as usize];
        }
    }
    false
}

fn count_adjacents(grid: &[Vec<bool>], i: isize, j: isize) -> usize {
    ADJACENTS
        .iter()
        .filter(|(di, dj)| get_adjacent(grid, i + di, j + dj))
        .count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect_vec())
        .collect_vec();

    let res = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|&(j, &c)| c && count_adjacents(&grid, i as isize, j as isize) < COMPARE)
                .count() as u64
        })
        .sum::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut positions = vec![];

    let mut grid = input
        .trim()
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.char_indices()
                .map(|(j, c)| {
                    let is_paper = c == '@';
                    if is_paper {
                        positions.push((i, j))
                    }
                    is_paper
                })
                .collect_vec()
        })
        .collect_vec();

    let mut res = 0;

    loop {
        let len = positions.len();

        positions.retain(|&(i, j)| {
            if count_adjacents(&grid, i as isize, j as isize) < COMPARE {
                grid[i][j] = false;
                false
            } else {
                true
            }
        });

        let modifs = (len - positions.len()) as u64;

        if modifs == 0 {
            break;
        }

        res += modifs;
    }

    Some(res)
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
        assert_eq!(result, Some(43));
    }
}
