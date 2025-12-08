use std::{collections::HashMap, ops::Sub};

use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day8;

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Pos(isize, isize, isize);

impl Sub for Pos {
    type Output = isize;

    fn sub(self, rhs: Self) -> Self::Output {
        (self.0 - rhs.0).pow(2) + (self.1 - rhs.1).pow(2) + (self.2 - rhs.2).pow(2)
    }
}

impl Day for Day8 {
    day_stuff!(8, "40", "25272", (usize, Vec<Pos>));

    fn part_1((amnt, input): Self::Input) -> Option<String> {
        let mut circuits = input
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, i))
            .collect::<HashMap<Pos, usize>>();

        let mut distances = input
            .iter()
            .enumerate()
            .flat_map(|(i, p)| input.iter().skip(i + 1).map(|&p2| (*p, p2, *p - p2)))
            .collect::<Vec<_>>();

        distances.sort_by_key(|(_, _, x)| *x);

        for (p1, p2, _dist) in distances.into_iter().take(amnt) {
            let target_circ = circuits.get(&p1).copied().unwrap();
            let replace_circ = circuits.get(&p2).copied().unwrap();

            if target_circ == replace_circ {
                continue;
            }

            for (_p, v) in circuits.iter_mut().filter(|(_, v)| **v == replace_circ) {
                *v = target_circ;
            }
        }

        let mut counts = circuits
            .into_iter()
            .fold(HashMap::new(), |mut acc, (_, c)| {
                *(acc.entry(c).or_insert(0)) += 1;
                acc
            })
            .into_iter()
            .map(|(_, count)| count)
            .collect::<Vec<_>>();

        counts.sort_by(|a, b| a.cmp(b).reverse());

        Some(
            counts
                .into_iter()
                .take(3)
                .fold(1, |acc, x| acc * x)
                .to_string(),
        )
    }

    fn part_2((_amnt, input): Self::Input) -> Option<String> {
        let mut circuits = input
            .iter()
            .enumerate()
            .map(|(i, &p)| (p, i))
            .collect::<HashMap<Pos, usize>>();

        let mut distances = input
            .iter()
            .enumerate()
            .flat_map(|(i, p)| input.iter().skip(i + 1).map(|&p2| (*p, p2, *p - p2)))
            .collect::<Vec<_>>();

        distances.sort_by_key(|(_, _, x)| *x);

        let mut last_2: Option<(Pos, Pos)> = None;

        for (p1, p2, _dist) in distances.into_iter() {
            let target_circ = circuits.get(&p1).copied().unwrap();
            let replace_circ = circuits.get(&p2).copied().unwrap();

            if target_circ == replace_circ {
                continue;
            }

            last_2 = Some((p1, p2));

            for (_p, v) in circuits.iter_mut().filter(|(_, v)| **v == replace_circ) {
                *v = target_circ;
            }
        }

        let (a, b) = last_2.unwrap();

        Some((a.0 * b.0).to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        let mut lines = input.lines();

        let amnt = lines.next().unwrap().parse::<usize>().unwrap();

        let poses = lines
            .map(|l| {
                let (x, r) = l.split_once(',').unwrap();
                let (y, z) = r.split_once(',').unwrap();
                Pos(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect();

        (amnt, poses)
    }
}
