use std::collections::{HashMap, HashSet};

use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day22;

fn next_secret(mut num: usize) -> usize {
    num = ((num * 64) ^ num) % 16777216;
    num = ((num / 32) ^ num) % 16777216;
    num = ((num * 2048) ^ num) % 16777216;
    num
}

fn secret_n_times(init: usize, times: usize) -> usize {
    let mut num = init;
    for _ in 0..times {
        num = next_secret(num);
    }
    num
}

fn get_all_four_unique_changes(init: usize, times: usize) -> HashMap<[isize; 4], usize> {
    let mut last = 0;
    let mut changes = Vec::with_capacity(times + 1);
    let mut curr = init;
    for _ in 0..times {
        curr = next_secret(curr);
        let val = curr % 10;
        changes.push(((val as isize) - (last as isize), val));
        last = val;
    }

    changes
        .windows(4)
        .fold(HashMap::with_capacity(times / 4), |mut acc, w| {
            let changes = [w[0].0, w[1].0, w[2].0, w[3].0];
            let final_val = w[3].1;
            if !acc.contains_key(&changes) {
                acc.insert(changes, final_val);
            }
            acc
        })
}

impl Day for Day22 {
    day_stuff!(22, "", "", Vec<usize>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .map(|init| secret_n_times(init, 2000))
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let change_to_val = input
            .into_iter()
            .map(|init| get_all_four_unique_changes(init, 2000))
            .collect::<Vec<_>>();
        let all_changes = change_to_val
            .iter()
            .flat_map(|h| h.keys().copied())
            .collect::<HashSet<_>>();

        let ans = all_changes
            .into_iter()
            .map(|c| {
                change_to_val
                    .iter()
                    .map(|h| h.get(&c).unwrap_or(&0))
                    .sum::<usize>()
            })
            .max()
            .unwrap();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|l| l.parse::<usize>().unwrap())
            .collect()
    }
}
