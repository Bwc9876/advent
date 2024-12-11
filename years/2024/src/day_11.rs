use std::collections::HashMap;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::num::{num_digits, split_num_once};

pub struct Day11;

fn do_blinks(stones: Vec<usize>, blinks: usize) -> usize {
    let l = stones.len();
    let mut stone_map = stones
        .into_iter()
        .fold(HashMap::with_capacity(l), |mut acc, stone| {
            acc.entry(stone).and_modify(|c| *c += 1).or_insert(1);
            acc
        });

    for _i in 0..blinks {
        let mut new_map = HashMap::with_capacity(stone_map.len() * 2);

        for (num, count) in stone_map {
            if num == 0 {
                *new_map.entry(1).or_insert(0) += count;
            } else {
                if num_digits(num) % 2 == 0 {
                    let (left, right) = split_num_once(num);
                    *new_map.entry(left).or_insert(0) += count;
                    *new_map.entry(right).or_insert(0) += count;
                } else {
                    *new_map.entry(num * 2024).or_insert(0) += count;
                }
            }
        }

        stone_map = new_map;
    }

    stone_map.into_values().sum::<usize>()
}

impl Day for Day11 {
    day_stuff!(11, "55312", "65601038650482", Vec<usize>);

    fn part_1(input: Self::Input) -> Option<String> {
        Some(do_blinks(input, 25).to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        Some(do_blinks(input, 75).to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .trim()
            .split(" ")
            .map(|n| n.parse::<usize>().unwrap())
            .collect()
    }
}
