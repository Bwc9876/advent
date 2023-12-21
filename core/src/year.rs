use crate::parser::{DP, Selection};

use super::MAX_DAY;

pub trait Year {
    const YEAR: usize;

    fn solve_day(day: usize, part: usize, input: Option<&str>) -> Option<String>;

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
            },
            Selection::Single(day) => {
                match dp.part {
                    Selection::All => {
                        Self::solve_day_both_parts(day, "");
                    },
                    Selection::Single(part) => {
                        Self::solve_day(day, part, input);
                    },
                }
            },
        }
    }
}
