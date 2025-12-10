use good_lp::Solution;
use good_lp::{Expression, ProblemVariables, SolverModel, Variable, default_solver, variable};
use itertools::Itertools;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|l| {
            let mut parts = l.split(' ').map(|s| {
                let mut iter = s.chars();
                iter.next();
                iter.next_back();
                iter
            });

            // ignore joltages
            parts.next_back();

            let goal = parts.next().unwrap().enumerate().fold(0u64, |acc, (i, c)| {
                if c == '#' { acc | (1 << i) } else { acc }
            });

            let buttons = parts
                .map(|chars| {
                    chars
                        .as_str()
                        .split(',')
                        .map(|s| s.parse::<u64>().unwrap())
                        .fold(0u64, |acc, cur| acc | (1 << cur))
                })
                .collect_vec();

            for i in 1..=buttons.len() {
                for btns in buttons.iter().combinations(i) {
                    let res = btns.iter().fold(0u64, |a, &&b| a ^ b);
                    if res == goal {
                        return i as u64;
                    }
                }
            }

            0
        })
        .sum::<u64>();

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|l| {
            let mut buttons = l
                .split(' ')
                .skip(1)
                .map(|s| {
                    let mut iter = s.chars();
                    iter.next();
                    iter.next_back();
                    iter.as_str()
                        .split(',')
                        .map(|s| s.parse::<u64>().unwrap())
                        .collect_vec()
                })
                .collect_vec();

            let joltages = buttons.pop().unwrap();

            let mut joltages_btns = vec![vec![]; joltages.len()];

            let mut problem = ProblemVariables::new();

            let vars = buttons
                .iter()
                .map(|_| variable().integer().min(0))
                .collect_vec();

            let y: Vec<Variable> = problem.add_all(vars);

            y.iter().zip(buttons.iter()).for_each(|(var, btn)| {
                btn.iter().for_each(|&b| {
                    joltages_btns[b as usize].push(var);
                });
            });

            let objective: Expression = y.iter().sum();

            let mut model = problem.minimise(objective).using(default_solver);

            for (joltage, jvars) in joltages.iter().zip(joltages_btns.iter()) {
                let expr: Expression = jvars.iter().cloned().sum();
                model = model.with(expr.eq(*joltage as u32));
            }

            let solution = model.solve().unwrap();

            let res = solution.eval(y.iter().sum::<Expression>());

            res.round() as u64
        })
        .sum();

    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}
