use std::collections::HashSet;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{dir::CARDINALS, pos::Position};

pub struct Day10;

#[derive(Clone, Debug)]
pub struct Tile(usize);

impl From<char> for Tile {
    fn from(value: char) -> Self {
        Self(value.to_string().parse().unwrap())
    }
}

pub type Grid = utils::grid::Grid<Tile>;

fn get_asc(g: &Grid, pos: Position, num: usize) -> impl Iterator<Item = Position> + use<'_> {
    g.relatives(pos, &CARDINALS)
        .filter_map(move |(_, r_pos, t)| if t.0 == num + 1 { Some(r_pos) } else { None })
}

impl Day for Day10 {
    day_stuff!(10, "36", "81", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let starts = input
            .iter()
            .filter_map(|(p, t)| if t.0 == 0 { Some(p) } else { None });

        let mut tot = 0;

        for start in starts {
            let mut num = 0;
            let mut nxt = vec![start];
            while num != 9 {
                nxt = nxt
                    .into_iter()
                    .flat_map(|o| get_asc(&input, o, num))
                    .collect::<Vec<_>>();
                num += 1;
            }

            let hsh = nxt.into_iter().collect::<HashSet<_>>();

            tot += hsh.len();
        }

        Some(tot.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let starts = input
            .iter()
            .filter_map(|(p, t)| if t.0 == 0 { Some(p) } else { None });

        let mut tot = 0;

        for start in starts {
            let mut num = 0;
            let mut nxt = vec![start];
            while num != 9 {
                nxt = nxt
                    .into_iter()
                    .flat_map(|o| get_asc(&input, o, num))
                    .collect::<Vec<_>>();
                num += 1;
            }

            tot += nxt.len();
        }

        Some(tot.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input.trim())
    }
}
