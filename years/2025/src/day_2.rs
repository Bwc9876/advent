use std::ops::RangeInclusive;

use advent_core::{Day, day_stuff, ex_for_day};
use utils::num::{num_digits, split_num_at, split_num_once};

pub struct Day2;

impl Day for Day2 {
    day_stuff!(2, "1227775554", "4174379265", Vec<RangeInclusive<usize>>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .flat_map(|r| {
                r.into_iter().filter(|x| {
                    let (l, r) = split_num_once(*x);
                    l == r
                })
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .flat_map(|r| {
                r.into_iter().filter(|x| {
                    let digs = num_digits(*x);
                    (2..=digs).filter(|n| digs.is_multiple_of(*n)).any(|n| {
                        let mut parts = Vec::with_capacity(n);
                        let mut current = *x;
                        let split_at = digs / n;
                        for _ in 0..n {
                            let (rest, part) = split_num_at(current, split_at as u32);
                            parts.push(part);
                            current = rest;
                        }
                        parts
                            .first()
                            .copied()
                            .is_some_and(|first| parts.into_iter().all(|x| x == first))
                    })
                })
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .split(',')
            .map(|r| {
                let (l, r) = r.split_once('-').unwrap();
                (l.trim().parse::<usize>().unwrap())..=(r.trim().parse::<usize>().unwrap())
            })
            .collect()
    }
}
