use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(8);
type Coords = (u64, u64, u64);

fn dist(a: &Coords, b: &Coords) -> u64 {
    let diff = (a.0.abs_diff(b.0), a.1.abs_diff(b.1), a.2.abs_diff(b.2));
    diff.0 * diff.0 + diff.1 * diff.1 + diff.2 * diff.2
}

pub fn part_one(input: &str) -> Option<u64> {
    let boxes: Vec<Coords> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut circuits: HashMap<usize, Vec<Coords>> =
        boxes.iter().map(|&b| vec![b]).enumerate().collect();
    let mut box_2_circuit: HashMap<Coords, usize> =
        boxes.iter().enumerate().map(|(a, &b)| (b, a)).collect();

    #[cfg(not(test))]
    let iterations = 1000;
    #[cfg(test)]
    let iterations = 10;

    boxes
        .iter()
        .tuple_combinations()
        .map(|(a, b)| (a, b, dist(a, b)))
        .sorted_unstable_by_key(|a| a.2)
        .take(iterations)
        .for_each(|(&a, &b, _)| {
            let (ca, cb) = (box_2_circuit[&a], box_2_circuit[&b]);
            if ca == cb {
                return;
            }
            let removed = circuits.remove(&cb).unwrap();
            for moved_box in &removed {
                *box_2_circuit.get_mut(moved_box).unwrap() = ca;
            }
            circuits.get_mut(&ca).unwrap().extend(removed);
        });

    let res = circuits
        .values()
        .map(|v| v.len() as u64)
        .sorted_unstable()
        .rev()
        .take(3)
        .product::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let boxes: Vec<Coords> = input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u64>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let mut circuits: HashMap<usize, Vec<Coords>> =
        boxes.iter().map(|&b| vec![b]).enumerate().collect();
    let mut box_2_circuit: HashMap<Coords, usize> =
        boxes.iter().enumerate().map(|(a, &b)| (b, a)).collect();

    let iter = boxes
        .iter()
        .tuple_combinations::<(&Coords, &Coords)>()
        .map(|(a, b)| (a, b, dist(a, b)))
        .sorted_unstable_by_key(|a| a.2);

    for (&a, &b, _) in iter {
        let (ca, cb) = (box_2_circuit[&a], box_2_circuit[&b]);
        if ca == cb {
            continue;
        }
        let removed = circuits.remove(&cb).unwrap();
        for moved_box in &removed {
            *box_2_circuit.get_mut(moved_box).unwrap() = ca;
        }
        circuits.get_mut(&ca).unwrap().extend(removed);

        if circuits.len() == 1 {
            return Some(a.0 * b.0);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
