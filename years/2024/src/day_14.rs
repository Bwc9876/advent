use std::{cmp::Ordering, collections::HashSet};

use advent_core::{day_stuff, ex_for_day, Day};
use regex::Regex;
use utils::{pos::Position, upos};

pub struct Day14;

fn robot_go(pos: Position, vel: Position, times: isize, bounds: (usize, usize)) -> Position {
    pos.add(&vel.multiply_comp(times)).bind(bounds).into()
}

impl Day for Day14 {
    day_stuff!(14, "12", "", ((usize, usize), Vec<(Position, Position)>));

    fn part_1((bounds, input): Self::Input) -> Option<String> {
        let times = 100;
        let (ur, ul, ll, lr) = input
            .into_iter()
            .map(move |(pos, vel)| robot_go(pos, vel, times, bounds))
            .fold((0, 0, 0, 0), move |mut acc, robo| {
                let is_upper = match robo.y.cmp(&(bounds.1 as isize / 2)) {
                    Ordering::Equal => None,
                    Ordering::Greater => Some(false),
                    Ordering::Less => Some(true),
                };
                let is_left = match robo.x.cmp(&(bounds.0 as isize / 2)) {
                    Ordering::Equal => None,
                    Ordering::Greater => Some(false),
                    Ordering::Less => Some(true),
                };
                if let (Some(is_upper), Some(is_left)) = (is_upper, is_left) {
                    let to_inc = match (is_upper, is_left) {
                        (true, true) => &mut acc.1,
                        (true, false) => &mut acc.0,
                        (false, true) => &mut acc.2,
                        (false, false) => &mut acc.3,
                    };

                    *to_inc += 1;
                }
                acc
            });

        Some((ur * ul * ll * lr).to_string())
    }

    fn part_2((bounds, input): Self::Input) -> Option<String> {
        let re = Regex::new(include_str!("da_tree.txt")).unwrap();

        if bounds != (101, 103) {
            // Im to lazy to account for other sizes, womp womp.
            return Some(String::new());
        }

        for i in 0..i32::MAX {
            let bots = input
                .iter()
                .map(move |r| robot_go(r.0, r.1, i as isize, bounds))
                .collect::<HashSet<_>>();

            let hay = (0..bounds.1)
                .flat_map(|y| {
                    let bots = &bots;
                    (0..bounds.0)
                        .map(move |x| {
                            let pos = upos!(x, y);
                            if bots.contains(&pos) {
                                'X'
                            } else {
                                '_'
                            }
                        })
                        .chain(['\n'])
                })
                .collect::<String>();

            if re.is_match(&hay) {
                return Some(i.to_string());
            }
        }

        panic!("No Tree Womp Womp")
    }

    fn parse_input(input: &str) -> Self::Input {
        let (bounds, rest) = input.trim().split_once("\n\n").unwrap();

        let mut bounds = bounds.split(',').map(|s| s.parse::<usize>().unwrap());

        let (x, y) = (bounds.next().unwrap(), bounds.next().unwrap());

        let input = rest
            .lines()
            .map(|l| {
                let (pr, vr) = l.split_once(" ").unwrap();
                let p = pr
                    .split_once("=")
                    .unwrap()
                    .1
                    .split(",")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();
                let v = vr
                    .split_once("=")
                    .unwrap()
                    .1
                    .split(",")
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<_>>();

                (Position::new(p[0], p[1]), Position::new(v[0], v[1]))
            })
            .collect();

        ((x, y), input)
    }
}
