use std::{cmp::Ordering, collections::HashSet};

use advent_core::{day_stuff, ex_for_day, Day};
use regex::Regex;
use utils::{ipos, pos::Position};

pub struct Day14;

fn robot_go(pos: Position, vel: Position, times: isize, bounds: Position) -> Position {
    let new_pos = pos.add(&vel.multiply_comp(times));
    let x_r = new_pos.x % bounds.x;
    let y_r = new_pos.y % bounds.y;
    ipos!(
        if x_r < 0 { x_r + bounds.x } else { x_r },
        if y_r < 0 { y_r + bounds.y } else { y_r }
    )
}

impl Day for Day14 {
    day_stuff!(14, "", "", Vec<(Position, Position)>);

    fn part_1(input: Self::Input) -> Option<String> {
        let bounds = Position::new(101, 103);
        let times = 100;
        let (ur, ul, ll, lr) = input
            .into_iter()
            .map(move |(pos, vel)| robot_go(pos, vel, times, bounds))
            .fold((0, 0, 0, 0), move |mut acc, robo| {
                let is_upper = match robo.y.cmp(&(bounds.y / 2)) {
                    Ordering::Equal => None,
                    Ordering::Greater => Some(false),
                    Ordering::Less => Some(true),
                };
                let is_left = match robo.x.cmp(&(bounds.x / 2)) {
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

        Some((dbg!(ur) * dbg!(ul) * dbg!(ll) * dbg!(lr)).to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let bounds = Position::new(101, 103);

        let re = Regex::new(include_str!("da_tree.txt")).unwrap();

        for i in 0..i32::MAX {
            let bots = input
                .iter()
                .map(move |r| robot_go(r.0, r.1, i as isize, bounds))
                .collect::<HashSet<_>>();

            let hay = (0..bounds.y)
                .flat_map(|y| {
                    let bots = &bots;
                    (0..bounds.x)
                        .map(move |x| {
                            let pos = Position::new(x, y);
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

        Some("FUCK".to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .trim()
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
            .collect()
    }
}
