use crate::parser::{Selection, DP};

use super::MAX_DAY;

pub trait Year {
    const YEAR: usize;

    fn solve_day(day: usize, part: usize, input: Option<&str>) -> Option<String>;

    fn bench_day(day: usize, part: usize, input: Option<&str>);

    fn solve_day_both_parts(day: usize, extra_indent: &str);

    fn solve_all_days() {
        println!("Year {}:", Self::YEAR);
        for day in 1..=MAX_DAY {
            Self::solve_day_both_parts(day, "  ");
        }
    }

    fn run_dp(input: Option<&str>, dp: DP) {
        match dp.day {
            Selection::All => {
                Self::solve_all_days();
            }
            Selection::Single(day) => match dp.part {
                Selection::All => {
                    Self::solve_day_both_parts(day, "");
                }
                Selection::Single(part) => {
                    Self::solve_day(day, part, input);
                }
            },
        }
    }

    fn bench_dp(input: Option<&str>, dp: DP) {
        match dp.day {
            Selection::Single(day) => match dp.part {
                Selection::Single(part) => {
                    Self::bench_day(day, part, input);
                }
                _ => panic!("Cannot bench all parts, sorry :("),
            },
            _ => panic!("Cannot bench all days, sorry :("),
        }
    }
}
