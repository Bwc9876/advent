use advent_core::{day_stuff, ex_for_day, Day};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

pub struct Day19;

impl Day for Day19 {
    day_stuff!(19, "", "", (HashSet<String>, Vec<String>));

    fn part_1(input: Self::Input) -> Option<String> {
        let (avail, desire) = input;
        let ans = desire
            .into_iter()
            .filter(|pat| {
                let pattern_ends = pat.len() + 1;
                let mut seen = HashMap::with_capacity(pattern_ends);
                seen.insert(0, 1);
                for end in 1..pattern_ends {
                    for a in avail.iter().filter(|p| end >= p.len()) {
                        let avail_start = end - a.len();
                        if &pat[avail_start..end] == a
                            && let Some(avail_start) = seen.get(&avail_start).copied()
                        {
                            *seen.entry(end).or_insert(0) += avail_start;
                        }
                    }
                }
                seen.get(&pat.len()).copied().is_some_and(|v| v != 0)
            })
            .count();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (avail, desire) = input;
        let ans = desire
            .into_par_iter()
            .map(|pat| {
                let pattern_ends = pat.len() + 1;
                let mut seen = HashMap::with_capacity(pattern_ends);
                seen.insert(0, 1);
                for end in 1..pattern_ends {
                    for a in avail.iter().filter(|p| end >= p.len()) {
                        let avail_start = end - a.len();
                        if &pat[avail_start..end] == a
                            && let Some(avail_start) = seen.get(&avail_start).copied()
                        {
                            *seen.entry(end).or_insert(0) += avail_start;
                        }
                    }
                }
                seen.get(&pat.len()).copied().unwrap_or(0)
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        let (avail, desire) = input.trim().split_once("\n\n").unwrap();
        (
            avail.split(", ").map(|s| s.to_string()).collect(),
            desire.split('\n').map(|s| s.to_string()).collect(),
        )
    }
}
