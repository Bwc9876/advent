use std::collections::HashSet;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::tiles;

pub struct Day8;

tiles!(Tile, [
    '.' => Empty,
], [
    Antenna(char)
], |c| {
    Self::Antenna(c)
});

type Grid = utils::grid::Grid<Tile>;

impl Day for Day8 {
    day_stuff!(8, "14", "34", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let all_antennas = input
            .iter()
            .filter_map(|(pos, t)| match *t {
                Tile::Empty => None,
                Tile::Antenna(c) => Some((pos, c)),
            })
            .collect::<Vec<_>>();

        let mut anti_nodes = HashSet::with_capacity(20);

        for (pos, freq) in all_antennas.iter() {
            all_antennas
                .iter()
                .filter(|(p, f)| p != pos && f == freq)
                .for_each(|(pos2, _)| {
                    let distance = pos2.sub(pos);

                    let anti_one = pos2.add(&distance);
                    let anti_two = pos.sub(&distance);

                    if input.in_bounds(&anti_one) {
                        anti_nodes.insert(anti_one);
                    }
                    if input.in_bounds(&anti_two) {
                        anti_nodes.insert(anti_two);
                    }
                })
        }

        Some(anti_nodes.len().to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let all_antennas = input
            .iter()
            .filter_map(|(pos, t)| match *t {
                Tile::Empty => None,
                Tile::Antenna(c) => Some((pos, c)),
            })
            .collect::<Vec<_>>();

        let mut anti_nodes = HashSet::with_capacity(20);

        for (pos, freq) in all_antennas.iter() {
            all_antennas
                .iter()
                .filter(|(p, f)| p != pos && f == freq)
                .for_each(|(pos2, _)| {
                    anti_nodes.insert(*pos);

                    let distance = pos2.sub(pos);

                    let mut anti_one = pos2.add(&distance);
                    let mut anti_two = pos.sub(&distance);

                    while input.in_bounds(&anti_one) {
                        anti_nodes.insert(anti_one);
                        anti_one = anti_one.add(&distance);
                    }
                    while input.in_bounds(&anti_two) {
                        anti_nodes.insert(anti_two);
                        anti_two = anti_two.sub(&distance);
                    }
                })
        }

        Some(anti_nodes.len().to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}
