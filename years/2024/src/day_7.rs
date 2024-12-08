use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day7;

impl Day for Day7 {
    day_stuff!(7, "3749", "11387", Vec<(i64, Vec<i64>)>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .filter_map(|(target, operands)| {
                let ps: u64 = 1 << (operands.len() - 1);
                for p in 0..=ps {
                    let res = operands.iter().enumerate().fold(0, |acc, (i, e)| {
                        if i == 0 {
                            acc + *e
                        } else if p & (1 << (i - 1)) != 0 {
                            acc * *e
                        } else {
                            acc + *e
                        }
                    });
                    if res == target {
                        return Some(target);
                    }
                }
                None
            })
            .sum::<i64>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .filter_map(|(target, operands)| {
                let ps: u64 = 3_u64.pow((operands.len() - 1) as u32);
                for p in 0..=ps {
                    let res = operands.iter().enumerate().fold(0, |acc, (i, e)| {
                        if i == 0 {
                            acc + *e
                        } else if p / 3_u64.pow((i - 1) as u32) % 3 == 1 {
                            acc * *e
                        } else if p / 3_u64.pow((i - 1) as u32) % 3 == 2 {
                            // Integer logartihm to prevent calls to to_string?
                            format!("{acc}{e}").parse().unwrap()
                        } else {
                            acc + *e
                        }
                    });
                    if res == target {
                        return Some(target);
                    }
                }
                None
            })
            .sum::<i64>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (eq, rest) = l.split_once(": ").unwrap();
                (
                    eq.parse().unwrap(),
                    rest.split(" ").map(|x| x.parse().unwrap()).collect(),
                )
            })
            .collect()
    }
}
