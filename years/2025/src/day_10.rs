use std::collections::VecDeque;

use advent_core::{day_stuff, ex_for_day, Day};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use good_lp::*;

pub struct Day10;

#[derive(Debug, Clone)]
pub struct Machine {
    target_state: usize,
    buttons: Vec<Vec<usize>>,
    target_counters: Vec<usize>,
}

impl Day for Day10 {
    day_stuff!(10, "7", "33", Vec<Machine>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .into_par_iter()
            .map(|mach| {
                let buttons = mach
                    .buttons
                    .iter()
                    .map(|b| b.iter().map(|n| 2_usize.pow(*n as u32)).sum())
                    .collect::<Vec<_>>();

                let mut queue: VecDeque<(usize, usize, Option<usize>)> =
                    VecDeque::with_capacity(30);
                queue.push_front((0, 0, None));
                let mut guy = None;
                while let Some((val, curr, prev)) = queue.pop_front() {
                    if val == mach.target_state {
                        guy = Some(curr);
                        break;
                    }

                    for butt in buttons.iter().copied() {
                        if prev.is_none_or(|v| v != butt) {
                            queue.push_back((val ^ butt, curr + 1, Some(butt)));
                        }
                    }
                }
                guy.expect("womp womp")
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let ans = input
            .into_par_iter()
            .map(|mach| {
                let mut vars = variables!();
                let press_vars = mach
                    .buttons
                    .iter()
                    .map(|_| vars.add(variable().min(0).integer()))
                    .collect::<Vec<_>>();

                let mut problem = vars
                    .minimise(press_vars.iter().sum::<Expression>())
                    .using(microlp);

                let mut exprs = vec![0.into_expression(); mach.target_counters.len()];
                for (i, butt) in mach.buttons.iter().enumerate() {
                    for &x in butt {
                        exprs[x] += press_vars[i];
                    }
                }

                for (e, j) in exprs.into_iter().zip(mach.target_counters) {
                    problem.add_constraint(e.eq(j as f64));
                }

                let sol = problem.solve().expect("womp womp");

                press_vars.iter().map(|&v| sol.value(v)).sum::<f64>() as usize
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let split = l.split(' ').collect::<Vec<_>>();
                let diag = split
                    .first()
                    .unwrap()
                    .chars()
                    .filter(|c| *c == '.' || *c == '#')
                    .enumerate()
                    .map(|(i, d)| {
                        (if d == '.' { 0_usize } else { 1_usize }) * 2_usize.pow(i as u32)
                    })
                    .sum();

                let buttons = split
                    .iter()
                    .take(split.len() - 1)
                    .skip(1)
                    .map(|b| {
                        b.trim_matches('(')
                            .trim_matches(')')
                            .split(',')
                            .map(|n| n.parse().unwrap())
                            .collect()
                    })
                    .collect();

                let target_counters = split
                    .last()
                    .unwrap()
                    .trim_matches('{')
                    .trim_matches('}')
                    .split(',')
                    .map(|n| n.parse().unwrap())
                    .collect();

                Machine {
                    target_state: diag,
                    buttons,
                    target_counters,
                }
            })
            .collect()
    }
}
