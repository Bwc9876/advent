use std::collections::{HashMap, HashSet, VecDeque};

use advent_core::{Day, day_stuff, ex_for_day};
use utils::{dir::Direction, pos::Position, tiles};

pub struct Day7;

tiles!(Tile, [
  'S' => Start,
  '^' => Splitter,
  '.' => Empty,
]);

type Grid = utils::grid::Grid<Tile>;

impl Day for Day7 {
    day_stuff!(7, "21", "40", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let (start, _) = input.iter().find(|(_, t)| **t == Tile::Start).unwrap();
        let mut queue = VecDeque::<Position>::new();
        let mut seen = HashSet::<Position>::with_capacity(input.size().1);

        queue.push_back(start);

        let mut splits = 0;

        while let Some(pos) = queue.pop_front() {
            if seen.contains(&pos) {
                continue;
            }
            seen.insert(pos);
            let next_pos = Position::new(pos.x, pos.y + 1);
            match input.get(next_pos) {
                Some(Tile::Splitter) => {
                    splits += 1;
                    queue.push_back(Position::new(pos.x + 1, pos.y));
                    queue.push_back(Position::new(pos.x - 1, pos.y));
                }
                Some(Tile::Empty) => {
                    queue.push_back(next_pos);
                }
                _ => {}
            }
        }

        Some(splits.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (start, _) = input.iter().find(|(_, t)| **t == Tile::Start).unwrap();
        let mut queue = VecDeque::<Position>::new();
        let mut seen = HashMap::<Position, usize>::new();

        seen.insert(start, 1);
        queue.push_front(start);

        let mut splits = 0;

        while let Some(pos) = queue.pop_front() {
            let amnt = seen.get(&pos).copied().unwrap_or_default();
            let next_pos = pos.move_dir(Direction::South);
            match input.get(next_pos) {
                Some(Tile::Splitter) => {
                    for (next_pos, _) in next_pos.relatives(&[Direction::West, Direction::East]) {
                        if let Some(curr) = seen.get_mut(&next_pos) {
                            *curr += amnt;
                        } else {
                            seen.insert(next_pos, amnt);
                            queue.push_back(next_pos);
                        }
                    }
                }
                Some(Tile::Empty) => {
                    if let Some(curr) = seen.get_mut(&next_pos) {
                        *curr += amnt;
                    } else {
                        seen.insert(next_pos, amnt);
                        queue.push_back(next_pos);
                    }
                }
                None => {
                    splits += amnt;
                }
                _ => {}
            }
        }

        Some(splits.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}
