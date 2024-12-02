
use advent_core::{Day, day_stuff, ex_for_day};
use utils::misc::{all_combos_remove_one, follows_diff_range, FollowRangeResult};

pub struct Day2;

fn line_valid(line: &[i64]) -> bool {
    let res = follows_diff_range(line, -3..4, true, false);
    matches!(res, FollowRangeResult::Increasing | FollowRangeResult::Decreasing)
}

impl Day for Day2 {

    day_stuff!(2, "2", "4", Vec<Vec<i64>>);

    fn part_1(input: Self::Input) -> Option<String> {
        Some(input.into_iter().filter(|v| line_valid(v)).count().to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        Some(input.into_iter().filter(|line| {
            line_valid(line) || 
            all_combos_remove_one(line)
            .any(|combo| {
                let v = combo.copied().collect::<Vec<_>>();
                line_valid(&v)
            })
        }).count().to_string())
    }
    
    fn parse_input(input: &str) -> Self::Input {
        input.split('\n').map(|l| l.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap()).collect()).collect()
    }
}
