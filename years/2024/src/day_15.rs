use std::collections::{HashMap, HashSet};

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{
    dir::{Direction, Movement},
    grid::Grid,
    pos::Position,
    tiles, upos,
};

pub struct Day15;

tiles!(Tile, [
    '.' => Empty,
    '@' => Robot,
    '#' => Wall,
    'O' => Box,
    '[' => BoxLeft,
    ']' => BoxRight,
]);

fn parse_instruction(c: char) -> Direction {
    match c {
        '^' => Direction::North,
        '<' => Direction::West,
        '>' => Direction::East,
        'v' => Direction::South,
        _ => unreachable!("For {c:?}"),
    }
}

type PosMap = HashMap<Position, Tile>;

fn movement(robo: Position, dir: Direction, map: &mut PosMap) -> Option<Position> {
    let mut next_pos = robo.add(&dir.get_kernel());
    let mut to_update = Vec::with_capacity(20);
    loop {
        let next_tile = map.get(&next_pos).unwrap();
        if *next_tile == Tile::Wall {
            return None;
        } else if *next_tile == Tile::Box {
            to_update.push(next_pos);
            next_pos = next_pos.add(&dir.get_kernel());
        } else {
            // Is Empty
            to_update.push(next_pos);
            break;
        }
    }
    assert!(!to_update.is_empty());
    let first = to_update.first().unwrap();
    *map.get_mut(first).unwrap() = Tile::Robot;
    *map.get_mut(&robo).unwrap() = Tile::Empty;
    if to_update.len() >= 2 {
        *map.get_mut(to_update.last().unwrap()).unwrap() = Tile::Box;
    }
    Some(*first)
}

fn movement_pt_2(robo: Position, dir: Direction, map: &mut PosMap) -> Option<Position> {
    let kern = dir.get_kernel();
    let mut to_check = HashSet::from_iter([robo.add(&kern)]);
    let mut to_update = Vec::with_capacity(40);
    to_update.push((robo.add(&kern), Tile::Robot));
    while !to_check.is_empty() {
        let mut new_check = HashSet::new();
        for check in to_check.into_iter() {
            let new_tile = *map.get(&check).unwrap();
            if new_tile == Tile::Wall {
                return None;
            } else if new_tile == Tile::BoxLeft && !dir.is_horizontal() {
                let my_new_pos = check.add(&kern);
                let r_pos = check.add(&upos!(1, 0));
                let r_new_pos = r_pos.add(&kern);
                to_update.push((my_new_pos, Tile::BoxLeft));
                to_update.push((r_new_pos, Tile::BoxRight));
                new_check.insert(my_new_pos);
                new_check.insert(r_new_pos);
            } else if new_tile == Tile::BoxRight && !dir.is_horizontal() {
                let my_new_pos = check.add(&kern);
                let l_pos = check.sub(&upos!(1, 0));
                let l_new_pos = l_pos.add(&kern);
                to_update.push((my_new_pos, Tile::BoxRight));
                to_update.push((l_new_pos, Tile::BoxLeft));
                new_check.insert(my_new_pos);
                new_check.insert(l_new_pos);
            } else if new_tile != Tile::Empty {
                to_update.push((check.add(&kern), new_tile));
                new_check.insert(check.add(&kern));
            }
        }
        to_check = new_check;
    }
    for (pos, tile) in to_update.into_iter().rev() {
        *map.get_mut(&pos).unwrap() = tile;
        *map.get_mut(&pos.sub(&kern)).unwrap() = Tile::Empty;
    }
    Some(robo.add(&kern))
}

fn gps(pos_map: &PosMap) -> usize {
    pos_map
        .iter()
        .filter_map(|(pos, tile)| {
            if matches!(*tile, Tile::Box | Tile::BoxLeft) {
                Some(100 * pos.y as usize + pos.x as usize)
            } else {
                None
            }
        })
        .sum()
}

impl Day for Day15 {
    day_stuff!(15, "", "", (Position, PosMap, Vec<Direction>));

    fn part_1(input: Self::Input) -> Option<String> {
        let (mut robo, mut pos_map, ins) = input;
        for i in ins {
            if let Some(new_pos) = movement(robo, i, dbg!(&mut pos_map)) {
                robo = new_pos;
            }
        }
        Some(gps(&pos_map).to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (mut robo, mut pos_map, ins) = input;
        for i in ins {
            if let Some(new_pos) = movement_pt_2(robo, i, &mut pos_map) {
                println!("Move success");
                robo = new_pos;
            }
        }
        Some(gps(&pos_map).to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        // TODO: Temp for pt 2
        let replace = input.replace('#', "##");
        let replace = replace.replace('.', "..");
        let replace = replace.replace('O', "[]");
        let replace = replace.replace('@', "@.");
        println!("{}", &replace);
        let (map, dirs) = &replace.trim().split_once("\n\n").unwrap();
        let dirs = dirs
            .split('\n')
            .flat_map(|l| l.chars().map(parse_instruction))
            .collect();

        let grid = Grid::<Tile>::parse(map);

        let robo = grid.find_tile(&Tile::Robot).unwrap();

        let pos_map = grid.iter().map(|(pos, tile)| (pos, *tile)).collect();

        (robo, pos_map, dirs)
    }
}
