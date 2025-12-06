use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day6;

#[derive(Debug, Clone)]
pub enum Prob {
    Add(Vec<usize>),
    Mul(Vec<usize>),
}

impl Day for Day6 {
    day_stuff!(6, "4277556", "3263827");

    fn part_1(input: Self::Input) -> Option<String> {
        let uno = input.lines().collect::<Vec<_>>();
        let dos = uno
            .iter()
            .take(uno.len() - 1)
            .map(|l| {
                l.split_whitespace()
                    .map(|p| p.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let ops = uno
            .iter()
            .skip(uno.len() - 1)
            .next()
            .unwrap()
            .split_whitespace()
            .collect::<Vec<_>>();
        let mut problems = Vec::with_capacity(uno.len());

        for i in 0..dos[0].len() {
            let all_operands = dos.iter().map(|d| d[i]).collect::<Vec<_>>();
            let operator = ops[i];
            let prob = if operator == "*" {
                Prob::Mul(all_operands)
            } else {
                Prob::Add(all_operands)
            };

            problems.push(prob);
        }

        let ans = problems
            .into_iter()
            .map(|p| match p {
                Prob::Add(l) => l.iter().sum::<usize>(),
                Prob::Mul(l) => l.iter().copied().reduce(|a, b| a * b).unwrap(),
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let mut cols: Vec<Vec<_>> = vec![];

        for (_, row) in input.lines().enumerate() {
            for (j, c) in row.chars().enumerate() {
                if let Some(r) = cols.get_mut(j) {
                    r.push(c);
                } else {
                    cols.push(vec![c]);
                }
            }
        }

        let mut curr_val = 0;
        let mut curr_op: Option<char> = None;
        let mut tot = 0;

        for col in cols {
            if let Some(op) = curr_op {
                if col.iter().all(|c| *c == ' ') {
                    tot += curr_val;
                    curr_val = 0_usize;
                    curr_op = None;
                } else {
                    let num = col
                        .iter()
                        .collect::<String>()
                        .trim()
                        .parse::<usize>()
                        .unwrap();
                    if op == '+' {
                        curr_val += num;
                    } else {
                        curr_val *= num;
                    }
                }
            } else {
                curr_op = Some(col.last().copied().unwrap());
                curr_val = col
                    .iter()
                    .take(col.len() - 1)
                    .collect::<String>()
                    .trim()
                    .parse::<usize>()
                    .unwrap();
            }
        }

        tot += curr_val;

        Some(tot.to_string())
    }
}
