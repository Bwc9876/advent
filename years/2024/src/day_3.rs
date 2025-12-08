use advent_core::{day_stuff, ex_for_day, Day};
use regex::RegexBuilder;

pub struct Day3;

const RE: &str = r"mul\((\d+),(\d+)\)";

fn re_do(i: &str) -> u64 {
    let re = RegexBuilder::new(RE).multi_line(true).build().unwrap();
    re.captures_iter(i)
        .map(|c| {
            c.get(1).unwrap().as_str().parse::<u64>().unwrap()
                * c.get(2).unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum::<u64>()
}

impl Day for Day3 {
    day_stuff!(3, "161", "48");

    fn part_1(input: Self::Input) -> Option<String> {
        Some(re_do(&input).to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        Some(
            input
                .split("do()")
                .map(|v| re_do(v.split("don't()").nth(0).unwrap()))
                .sum::<u64>()
                .to_string(),
        )
    }
}
