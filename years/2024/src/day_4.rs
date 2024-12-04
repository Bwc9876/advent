use advent_core::{day_stuff, ex_for_day, Day};
use utils::{
    dir::{Movement, ALL_8},
    grid::Grid,
    kern,
    pos::Position,
};

pub struct Day4;

const SEARCH: &str = "XMAS";

const PART_2_FORWARD_DIAG: [Position; 2] = kern![NE, SW];
const PART_2_BACKWARD_DIAG: [Position; 2] = kern![NW, SE];

fn check_diag_good(grid: &Grid<char>, pos: Position, kerns: &[impl Movement]) -> bool {
    grid.relatives_strict(pos, kerns).is_some_and(|relatives| {
        let (_, _, x) = relatives[0];
        let (_, _, y) = relatives[1];
        (*x == 'S' || *x == 'M') && (*y == 'S' || *y == 'M') && (*x != *y)
    })
}

impl Day for Day4 {
    day_stuff!(4, "18", "9", Grid::<char>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .iter()
            .map(|(pos, c)| {
                if SEARCH.starts_with(*c) {
                    ALL_8
                        .iter()
                        .filter(|dir| {
                            let kerns = dir.repeat(1..(SEARCH.len() as isize)).collect::<Vec<_>>();

                            input
                                .relatives_strict(pos, &kerns)
                                .is_some_and(|relatives| {
                                    relatives
                                        .into_iter()
                                        .map(|(_, _, c)| *c)
                                        .zip(SEARCH[1..].chars())
                                        .all(|(l, r)| l == r)
                                })
                        })
                        .count()
                } else {
                    0
                }
            })
            .sum::<usize>();
        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let ans = input
            .iter()
            .filter(|(pos, c)| {
                **c == 'A'
                    && check_diag_good(&input, *pos, &PART_2_FORWARD_DIAG)
                    && check_diag_good(&input, *pos, &PART_2_BACKWARD_DIAG)
            })
            .count();
        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        Grid::parse(input)
    }
}
