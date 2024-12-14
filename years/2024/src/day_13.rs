use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day13;

#[derive(Debug, Clone, Copy)]
pub struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    goal: (isize, isize),
}

impl Machine {
    pub fn parse(raw: &str) -> Self {
        let mut l = raw.lines();
        let (ax, ay) = l
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (bx, by) = l
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (ax, ay) = (
            ax.split_once("+").unwrap().1.parse::<isize>().unwrap(),
            ay.split_once("+").unwrap().1.parse::<isize>().unwrap(),
        );

        let (bx, by) = (
            bx.split_once("+").unwrap().1.parse::<isize>().unwrap(),
            by.split_once("+").unwrap().1.parse::<isize>().unwrap(),
        );

        let (px, py) = l
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();
        let (px, py) = (
            px.split_once("=").unwrap().1.parse::<isize>().unwrap(),
            py.split_once("=").unwrap().1.parse::<isize>().unwrap(),
        );

        Self {
            a: (ax, ay),
            b: (bx, by),
            goal: (px, py),
        }
    }

    pub fn presses_needed_for_prizes(&self) -> Option<isize> {
        let a_presses = (self.b.0 * self.goal.1 - self.b.1 * self.goal.0) as f64
            / (self.b.0 * self.a.1 - self.b.1 * self.a.0) as f64;

        let b_left = self.goal.0 as f64 - self.a.0 as f64 * a_presses;
        let b_presses = b_left / self.b.0 as f64;

        if a_presses % 1.0 == 0.0 && b_presses % 1.0 == 0.0 {
            Some((3.0 * a_presses + b_presses) as isize)
        } else {
            None
        }
    }
}

impl Day for Day13 {
    day_stuff!(13, "", "", Vec<Machine>);

    fn part_1(input: Self::Input) -> Option<String> {
        Some(
            input
                .iter()
                .filter_map(Machine::presses_needed_for_prizes)
                .sum::<isize>()
                .to_string(),
        )
    }

    fn part_2(mut input: Self::Input) -> Option<String> {
        input.iter_mut().for_each(|m| {
            m.goal.0 += 10000000000000;
            m.goal.1 += 10000000000000;
        });

        Some(
            input
                .iter()
                .filter_map(Machine::presses_needed_for_prizes)
                .sum::<isize>()
                .to_string(),
        )
    }

    fn parse_input(input: &str) -> Self::Input {
        input.split("\n\n").map(Machine::parse).collect()
    }
}
