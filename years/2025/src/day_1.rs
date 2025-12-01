use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Dir {
    Left,
    Right,
}

#[derive(Clone, Debug)]
pub struct Rot {
    dir: Dir,
    amnt: isize,
}

impl Day for Day1 {
    fn part_1(input: Self::Input) -> Option<String> {
        let mut curr = 50;
        let mut at_0 = 0;
        for rot in input {
            let amnt = if rot.dir == Dir::Left {
                -rot.amnt
            } else {
                rot.amnt
            };
            curr = (curr + amnt).rem_euclid(100);
            if curr == 0 {
                at_0 += 1;
            }
        }

        Some(at_0.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let mut curr = 50;
        let mut at_0 = 0;
        for rot in input {
            let amnt = if rot.dir == Dir::Left {
                -rot.amnt
            } else {
                rot.amnt
            };
            at_0 += if rot.dir == Dir::Right {
                (curr + rot.amnt) / 100
            } else {
                (if curr == 0 { 0 } else { 100 - curr } + rot.amnt) / 100
            };
            curr = (curr + amnt).rem_euclid(100);
        }

        Some(at_0.to_string())
    }

    day_stuff!(1, "3", "6", Vec<Rot>);

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (dir, amnt) = l.split_at(1);
                Rot {
                    dir: if dir == "L" { Dir::Left } else { Dir::Right },
                    amnt: amnt.parse().unwrap(),
                }
            })
            .collect()
    }
}
