use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{
    dir::{Direction, Movement},
    pos::Position,
    tiles,
};

pub struct Day16;

tiles!(Tile, [
    '.' => Open,
    '#' => Wall,
    'S' => Start,
    'E' => End,
]);

type Grid = utils::grid::Grid<Tile>;

#[derive(Clone, Eq, PartialEq)]
struct DState {
    cost: usize,
    vert: (Position, Direction),
    prev: Vec<Position>,
}

impl Ord for DState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for DState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Day for Day16 {
    day_stuff!(16, "11048", "64", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let start_pos = input.find_tile(&Tile::Start).unwrap();
        let end_pos = input.find_tile(&Tile::End).unwrap();

        let mut queue = BinaryHeap::new();

        queue.push(DState {
            vert: (start_pos, Direction::East),
            cost: 0,
            prev: vec![],
        });

        let mut visited = HashMap::<(Position, Direction), usize>::new();

        while let Some(DState {
            vert: (pos, dir),
            cost,
            prev: _,
        }) = queue.pop()
        {
            if pos == end_pos {
                return Some(cost.to_string());
            }

            if visited
                .get(&(pos, dir))
                .is_some_and(|min_score| *min_score < cost)
            {
                continue;
            }

            for (next_dir, score) in input
                .relatives(pos, &[dir, dir.ninety_deg(true), dir.ninety_deg(false)])
                .filter_map(|(next_dir, _, t)| {
                    if *t != Tile::Wall {
                        Some((next_dir, if next_dir == dir { 1 } else { 1000 }))
                    } else {
                        None
                    }
                })
            {
                let next_state = DState {
                    cost: cost + score,
                    vert: (
                        if dir == next_dir {
                            pos.add(&dir.get_kernel())
                        } else {
                            pos
                        },
                        next_dir,
                    ),
                    prev: vec![],
                };

                if next_state.cost < *visited.get(&next_state.vert).unwrap_or(&usize::MAX) {
                    *visited.entry(next_state.vert).or_insert(usize::MAX) = next_state.cost;
                    queue.push(next_state);
                }
            }
        }

        panic!("No Solution!!!")
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let start_pos = input.find_tile(&Tile::Start).unwrap();
        let end_pos = input.find_tile(&Tile::End).unwrap();

        let mut queue = BinaryHeap::with_capacity(input.width());

        queue.push(DState {
            vert: (start_pos, Direction::East),
            cost: 0,
            prev: vec![],
        });

        let mut visited = HashMap::<(Position, Direction), usize>::with_capacity(input.width());

        let mut all_good = HashSet::with_capacity(500);

        let mut found_min = None;

        while let Some(DState {
            vert: (pos, dir),
            cost,
            prev,
        }) = queue.pop()
        {
            if pos == end_pos {
                //return Some(cost.to_string());
                if found_min.is_none_or(|s| s == cost) {
                    all_good.extend(prev.into_iter());
                    all_good.insert(end_pos);
                    found_min = Some(cost);
                    continue;
                } else {
                    break;
                }
            }

            if visited
                .get(&(pos, dir))
                .is_some_and(|min_score| *min_score < cost)
            {
                continue;
            }

            for (next_dir, score) in input
                .relatives(pos, &[dir, dir.ninety_deg(true), dir.ninety_deg(false)])
                .filter_map(|(next_dir, _, t)| {
                    if *t != Tile::Wall {
                        Some((next_dir, if next_dir == dir { 1 } else { 1000 }))
                    } else {
                        None
                    }
                })
            {
                let mut next_prev = prev.clone();
                next_prev.push(pos);
                let next_state = DState {
                    cost: cost + score,
                    vert: (
                        if dir == next_dir {
                            pos.add(&dir.get_kernel())
                        } else {
                            pos
                        },
                        next_dir,
                    ),
                    prev: next_prev,
                };

                if next_state.cost <= *visited.get(&next_state.vert).unwrap_or(&usize::MAX) {
                    *visited.entry(next_state.vert).or_insert(usize::MAX) = next_state.cost;
                    queue.push(next_state);
                }
            }
        }

        if all_good.contains(&end_pos) {
            Some(all_good.len().to_string())
        } else {
            panic!("No Solution!!!")
        }
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}
