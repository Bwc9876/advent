use advent_core::{Day, day_stuff, ex_for_day};
use utils::{dir::ALL_8, tiles};

pub struct Day4;

tiles!(Tile, [
  '@' => Paper,
  '.' => Empty
]);

type Grid = utils::grid::Grid<Tile>;

impl Day for Day4 {
    day_stuff!(4, "13", "43", Grid);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .iter()
            .filter(|(_, t)| **t == Tile::Paper)
            .filter(|(pos, _)| {
                input
                    .relatives(*pos, &ALL_8)
                    .filter(|(_, _, t)| **t == Tile::Paper)
                    .count()
                    < 4
            })
            .count();

        Some(ans.to_string())
    }

    fn part_2(mut input: Self::Input) -> Option<String> {
        let mut i = 0;

        loop {
            let next = input
                .iter()
                .filter(|(_, t)| **t == Tile::Paper)
                .filter(|(pos, _)| {
                    input
                        .relatives(*pos, &ALL_8)
                        .filter(|(_, _, t)| **t == Tile::Paper)
                        .count()
                        < 4
                })
                .map(|(p, _)| p)
                .collect::<Vec<_>>();

            if next.is_empty() {
                break;
            } else {
                i += next.len();
                for pos in next {
                    *(input.get_mut(pos).unwrap()) = Tile::Empty;
                }
            }
        }

        Some(i.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}
