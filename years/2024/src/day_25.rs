use std::collections::HashSet;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{tiles, upos};

pub struct Day25;

tiles!(Tile, [
    '#' => Fill,
    '.' => Empty,
]);

type Grid = utils::grid::Grid<Tile>;

impl Day for Day25 {
    day_stuff!(25, "3", "🥳", (HashSet<[u8; 5]>, HashSet<[u8; 5]>));

    fn part_1((locks, keys): Self::Input) -> Option<String> {
        let ans = locks
            .into_iter()
            .flat_map(|l| {
                keys.iter()
                    .filter(move |k| l.iter().zip(k.iter()).all(|(l, k)| *k <= (5 - *l)))
            })
            .count();

        Some(ans.to_string())
    }

    fn part_2(_input: Self::Input) -> Option<String> {
        Some("🥳".to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        let mut locks = HashSet::new();
        let mut keys = HashSet::new();

        for grid in input.trim().split("\n\n").map(Grid::parse) {
            let code = grid
                .iter_cols()
                .map(|col| (col.filter(|t| **t == Tile::Fill).count() - 1) as u8)
                .collect::<Vec<_>>();

            let code = [code[0], code[1], code[2], code[3], code[4]];

            if grid.get(upos!(0, 0)).is_some_and(|t| *t == Tile::Fill) {
                locks.insert(code);
            } else {
                keys.insert(code);
            }
        }

        (locks, keys)
    }
}
