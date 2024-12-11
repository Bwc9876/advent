use std::collections::HashMap;

use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day11;

impl Day for Day11 {
    day_stuff!(11, "55312", "65601038650482", Vec<u128>);

    fn part_1(input: Self::Input) -> Option<String> {
        let l = input.len();
        let mut stone_map = input
            .into_iter()
            .fold(HashMap::with_capacity(l), |mut acc, stone| {
                acc.entry(stone).and_modify(|c| *c += 1).or_insert(1);
                acc
            });

        for _i in 0..25 {
            let mut new_map = HashMap::with_capacity(stone_map.len());

            for (num, count) in stone_map {
                if num == 0 {
                    new_map
                        .entry(1)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                } else {
                    let ss = num.to_string();
                    let l = ss.len();
                    if l % 2 == 0 {
                        let (num1, num2) =
                            (ss[..l / 2].parse().unwrap(), ss[l / 2..].parse().unwrap());
                        new_map
                            .entry(num1)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                        new_map
                            .entry(num2)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                    } else {
                        new_map
                            .entry(num * 2024)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                    }
                }
            }

            stone_map = new_map;
        }

        Some(stone_map.values().sum::<u128>().to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let l = input.len();
        let mut stone_map = input
            .into_iter()
            .fold(HashMap::with_capacity(l), |mut acc, stone| {
                acc.entry(stone).and_modify(|c| *c += 1).or_insert(1);
                acc
            });

        for _i in 0..75 {
            let mut new_map = HashMap::with_capacity(stone_map.len());

            for (num, count) in stone_map {
                if num == 0 {
                    new_map
                        .entry(1)
                        .and_modify(|c| *c += count)
                        .or_insert(count);
                } else {
                    let ss = num.to_string();
                    let l = ss.len();
                    if l % 2 == 0 {
                        let (num1, num2) =
                            (ss[..l / 2].parse().unwrap(), ss[l / 2..].parse().unwrap());
                        new_map
                            .entry(num1)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                        new_map
                            .entry(num2)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                    } else {
                        new_map
                            .entry(num * 2024)
                            .and_modify(|c| *c += count)
                            .or_insert(count);
                    }
                }
            }

            stone_map = new_map;
        }

        Some(stone_map.values().sum::<u128>().to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .trim()
            .split(" ")
            .map(|n| n.parse::<u128>().unwrap())
            .collect()
    }
}
