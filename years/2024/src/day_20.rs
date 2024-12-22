use std::collections::HashMap;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{dir::CARDINALS, pos::Position, tiles};

pub struct Day20;

tiles!(Tile, [
    '.' => Open,
    '#' => Wall,
    'S' => Start,
    'E' => End,
]);

type Grid = utils::grid::Grid<Tile>;

impl Day for Day20 {
    day_stuff!(20, "", "", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let end_pos = input.find_tile(&Tile::End).unwrap();
        let start_pos = input.find_tile(&Tile::Start).unwrap();
        let mut costs = HashMap::with_capacity(100);
        let mut curs = end_pos;
        let mut cost = 0;
        while curs != start_pos {
            costs.insert(curs, cost);
            let (_, next_pos, _) = input
                .relatives(curs, &CARDINALS)
                .filter(|(_, p, t)| **t != Tile::Wall && !costs.contains_key(p))
                .next()
                .unwrap();
            curs = next_pos;
            cost += 1;
        }
        costs.insert(start_pos, cost);

        let mut cheat_set = HashMap::<(Position, Position), usize>::with_capacity(costs.len());
        for (pos_a, cost_a) in costs.iter() {
            for (pos_b, cost_b) in costs.iter() {
                if cost_b < cost_a {
                    let diff = *cost_a - *cost_b;
                    let dist = pos_a.manhattan(&pos_b).abs() as usize;
                    if dist <= 2 {
                        cheat_set.insert((*pos_a, *pos_b), diff - dist);
                    }
                }
            }
        }

        let ans = cheat_set.values().filter(|c| **c >= 100).count();

        dbg!(cheat_set.len());

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let end_pos = input.find_tile(&Tile::End).unwrap();
        let start_pos = input.find_tile(&Tile::Start).unwrap();
        let mut costs = HashMap::with_capacity(100);
        let mut curs = end_pos;
        let mut cost = 0;
        while curs != start_pos {
            costs.insert(curs, cost);
            let (_, next_pos, _) = input
                .relatives(curs, &CARDINALS)
                .filter(|(_, p, t)| **t != Tile::Wall && !costs.contains_key(p))
                .next()
                .unwrap();
            curs = next_pos;
            cost += 1;
        }
        costs.insert(start_pos, cost);

        let mut cheat_set = HashMap::<(Position, Position), usize>::with_capacity(costs.len());
        for (pos_a, cost_a) in costs.iter() {
            for (pos_b, cost_b) in costs.iter() {
                if cost_b < cost_a {
                    let diff = *cost_a - *cost_b;
                    let dist = pos_a.manhattan(&pos_b).abs() as usize;
                    if dist <= 20 {
                        cheat_set.insert((*pos_a, *pos_b), diff - dist);
                    }
                }
            }
        }

        let ans = cheat_set.values().filter(|c| **c >= 100).count();

        dbg!(cheat_set.len());

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}
