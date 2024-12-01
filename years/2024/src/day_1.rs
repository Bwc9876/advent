
use core::{Day, day_stuff, ex_for_day};

pub struct Day1;

impl Day for Day1 {

    day_stuff!(1, "", "");

    fn part_1(_input: Self::Input) -> Option<String> {
        Some(_input.to_string())
    }

    fn part_2(_input: Self::Input) -> Option<String> {
        None
    }
}
