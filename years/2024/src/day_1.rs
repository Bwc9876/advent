use advent_core::{day_stuff, ex_for_day, Day};
use utils::misc::counts;

pub struct Day1;

impl Day for Day1 {
    day_stuff!(1, "11", "31", (Vec<i32>, Vec<i32>));

    fn part_1(input: Self::Input) -> Option<String> {
        let (mut l, mut r) = input;
        l.sort_unstable();
        r.sort_unstable();

        Some(
            l.into_iter()
                .zip(r)
                .map(|(l, r)| (l - r).abs())
                .sum::<i32>()
                .to_string(),
        )
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (l, r) = input;
        let apr = counts(r.into_iter());
        Some(
            l.into_iter()
                .map(|l| l as u64 * apr.get(&l).unwrap_or(&0))
                .sum::<u64>()
                .to_string(),
        )
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .split("\n")
            .map(|line| {
                let mut split = line.trim().split_ascii_whitespace();
                let left = split.next().unwrap().parse::<i32>().unwrap();
                let right = split.next().unwrap().parse::<i32>().unwrap();
                (left, right)
            })
            .collect()
    }
}
