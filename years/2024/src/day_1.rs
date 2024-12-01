
use std::collections::HashMap;

use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day1;

impl Day for Day1 {

    day_stuff!(1, "11", "31", (Vec<i32>, Vec<i32>));

    fn part_1(input: Self::Input) -> Option<String> {
        let (mut l, mut r) = input;
        l.sort_unstable();
        r.sort_unstable();
        Some(l.into_iter().zip(r.into_iter()).map(|(l, r)| (l - r).abs()).sum::<i32>().to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (l, r) = input;
        let apr = r.into_iter().fold(HashMap::new(), |mut a, curr| {
            a.entry(curr).and_modify(|x| { *x += 1 }).or_insert(1);
            a
        });
        Some(l.into_iter().map(|l| l * apr.get(&l).unwrap_or(&0)).sum::<i32>().to_string())
    }
    
    fn parse_input(input: &str) -> Self::Input {
        input.split("\n").map(|l| {
            let mut l = l.trim().split_ascii_whitespace();
            (l.next().map(|x| x.parse::<i32>().unwrap()).unwrap(), l.next().map(|x| x.parse::<i32>().unwrap()).unwrap())
        }).collect::<(Vec<_>, Vec<_>)>()
    }
    
}
