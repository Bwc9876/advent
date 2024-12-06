use std::collections::HashSet;

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{dir::Direction, pos::Position, prelude::GridCursor, tiles};

pub struct Day6;

tiles!(Tile, [
'.' => Floor,
'#' => Wall,
'^' => GuardStart
]);

impl Tile {
    fn is_open(&self) -> bool {
        matches!(self, Self::Floor | Self::GuardStart)
    }
}

type Grid = utils::grid::Grid<Tile>;

/// Returns true if we get into a loop
fn crawl_grid_with_obs(mut curs: GridCursor<Tile, Direction>, obs_pos: Position) -> bool {
    let mut visited = HashSet::new();

    while let Some((pos, tile)) = curs.peek_forward() {
        visited.insert((curs.pos, curs.dir));
        if tile.is_open() && pos != obs_pos {
            curs.move_forward();
        } else {
            curs.turn(true);
        }

        if visited.contains(&(curs.pos, curs.dir)) {
            return true;
        }
    }

    false
}

impl Day for Day6 {
    day_stuff!(6, "41", "6", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let mut curs = input
            .cursor_at(&Tile::GuardStart, Direction::North)
            .unwrap();

        let mut visited = HashSet::with_capacity(input.width() * input.height());

        while let Some((_next_pos, tile)) = curs.peek_forward() {
            visited.insert(curs.pos);
            if tile.is_open() {
                curs.move_forward();
            } else {
                curs.turn(true);
            }
        }

        // While let means we'll be missing one
        visited.insert(curs.pos);

        Some(visited.len().to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let mut curs = input
            .cursor_at(&Tile::GuardStart, Direction::North)
            .unwrap();

        let start_curs = curs;

        let mut obs = HashSet::with_capacity(input.width() * input.height());

        while let Some((possible, tile)) = curs.peek_forward() {
            if !obs.contains(&possible) {
                let curs_2 = start_curs;
                if crawl_grid_with_obs(curs_2, possible) {
                    obs.insert(possible);
                }
            }

            if tile.is_open() {
                curs.move_forward();
            } else {
                curs.turn(true);
            }
        }

        Some(obs.len().to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}

// - ORIGINAL PART 2 : BRUTE FORCE APPROACH (Im so sad i had to do this) -

// let mut c = 0;

//         let start_pos = input
//             .iter()
//             .find_map(|(pos, c)| {
//                 if matches!(c, Tile::GuardStart) {
//                     Some(pos)
//                 } else {
//                     None
//                 }
//             })
//             .unwrap();

//         for x in 0..input.width() {
//             for y in 0..input.height() {
//                 let pos = mpos!(x as isize, y as isize);
//                 if matches!(input.get(pos).unwrap(), Tile::Floor) {
//                     let mut i2 = input.clone();
//                     i2.iter_mut().for_each(|(pt, r)| if pt == pos { *r = Tile::Wall; });
//                     let mut curs = GridCursor::new(&i2, start_pos, Direction::North);

//                     let mut visited = HashSet::with_capacity(i2.width());

//                     loop {
//                         visited.insert((curs.pos, curs.dir));
//                         if let Some(val_ahead) = i2.get(curs.pos.move_dir(curs.dir)) {
//                             if val_ahead.is_open() {
//                                 curs.move_forward();
//                             } else {
//                                 curs.turn(true);
//                             }
//                             if visited.contains(&(curs.pos, curs.dir)) {
//                                 c += 1;
//                                 break;
//                             }
//                         } else {
//                             break;
//                         }
//                     }
//                 }
//             }
//         }

//         Some(c.to_string())
