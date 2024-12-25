use std::collections::HashSet;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{
    dir::{Direction, Movement, CARDINALS},
    pos::Position,
    prelude::GridCursor,
};

pub struct Day12;

type Grid = utils::grid::Grid<char>;

fn trace_perim(
    grid: &Grid,
    starting_pos: Position,
    initial_dir: Direction,
    c: char,
) -> (usize, HashSet<Position>) {
    let mut perims = HashSet::with_capacity(100);
    let mut turns = 0;
    let mut curs = GridCursor::new(&grid, starting_pos, initial_dir);
    let mut init = false;

    while (curs.pos, curs.dir) != (starting_pos, initial_dir) || !init {
        perims.insert(curs.pos);

        let look_left = curs.pos.add(&curs.dir.ninety_deg(false).get_kernel());

        if grid.get(look_left).is_some_and(|c2| *c2 == c) {
            // Interior corner, turn left!
            curs.turn(false);
            curs.move_forward();
            turns += 1;
        } else if curs.peek_forward().is_none_or(|(_, c2)| *c2 != c) {
            // Exterior corner, turn right!
            curs.turn(true);
            turns += 1;
        } else {
            // Valid, keep going!
            curs.move_forward();
        }

        init = true;
    }

    (turns, perims)
}

impl Day for Day12 {
    fn part_1(input: Self::Input) -> Option<String> {
        let mut visited = HashSet::with_capacity(50);
        let mut total = 0;

        for (pos, c) in input.iter() {
            if !visited.contains(&pos) {
                let mut flood = vec![pos];

                let (mut area, mut perim) = (0, 0);

                while !flood.is_empty() {
                    let mut next = vec![];

                    for pos2 in flood.iter() {
                        if let Some(c2) = input.get(*pos2)
                            && c2 == c
                            && !visited.contains(pos2)
                        {
                            area += 1;
                            let mut rel = input
                                .relatives(*pos2, &CARDINALS)
                                .filter_map(
                                    |(_, pos3, c3)| if c2 == c3 { Some(pos3) } else { None },
                                )
                                .collect::<Vec<_>>();

                            if rel.len() != 4 {
                                perim += 4 - rel.len();
                            }

                            visited.insert(*pos2);
                            next.append(&mut rel);
                        }
                    }

                    flood = next;
                }

                total += area * perim;

                visited.insert(pos);
            }
        }

        Some(total.to_string())
    }

    // TODO: Still working on the ""nice"" way of doing this one
    fn part_2(input: Self::Input) -> Option<String> {
        let mut visited = HashSet::with_capacity(50);
        let mut shapes = Vec::<(usize, usize, HashSet<Position>)>::with_capacity(30);

        for (pos, c) in input.iter() {
            if !visited.contains(&pos) {
                let (turns, perimeters) = trace_perim(&input, pos, Direction::East, *c);

                let mut all_tiles = HashSet::with_capacity(turns);

                let mut flood = vec![pos];

                while !flood.is_empty() {
                    let mut next = vec![];

                    for pos2 in flood.iter() {
                        if let Some(c2) = input.get(*pos2)
                            && c2 == c
                            && !visited.contains(pos2)
                        {
                            all_tiles.insert(*pos2);

                            let mut rel = input
                                .relatives(*pos2, &CARDINALS)
                                .filter_map(
                                    |(_, pos3, c3)| if c2 == c3 { Some(pos3) } else { None },
                                )
                                .collect::<Vec<_>>();

                            if !perimeters.contains(pos2) {
                                let _inner_shape = if input
                                    .get(pos2.add(&Direction::East.get_kernel()))
                                    .is_some_and(|c2| c2 != c)
                                {
                                    Some(trace_perim(&input, *pos2, Direction::North, *c))
                                } else if input
                                    .get(pos2.add(&Direction::North.get_kernel()))
                                    .is_some_and(|c2| c2 != c)
                                {
                                    Some(trace_perim(&input, *pos2, Direction::East, *c))
                                } else {
                                    None
                                };
                            }

                            visited.insert(*pos2);
                            next.append(&mut rel);
                        }
                    }

                    flood = next;
                }

                shapes.push((all_tiles.len(), turns, all_tiles))
            }
        }

        Some(
            shapes
                .into_iter()
                .map(|(area, turns, _)| area * turns)
                .sum::<usize>()
                .to_string(),
        )
    }

    day_stuff!(12, "1930", "1206", Grid);

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input.trim())
    }
}
