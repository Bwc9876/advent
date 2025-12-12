use advent_core::{day_stuff, ex_for_day, Day};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use utils::{grid::Grid, pos::Position, tiles, upos, yippee};

pub struct Day12;

tiles!(Tile, [
    '#' => Filled,
    '.' => Empty,
]);

type Present = utils::grid::Grid<Tile>;
type Input = (Vec<[Vec<Position>; 4]>, Vec<((usize, usize), Vec<usize>)>);

pub const KERNS: [Position; 9] = [
    Position::new(0, 0),
    Position::new(0, -1),
    Position::new(0, 1),
    Position::new(1, 0),
    Position::new(-1, 0),
    Position::new(1, 1),
    Position::new(1, -1),
    Position::new(-1, 1),
    Position::new(-1, -1),
];

fn rot_pos(Position { x, y }: Position) -> Position {
    let (x, y) = match (x, y) {
        (0, 0) => (0, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 1) => (1, -1),
        (1, -1) => (-1, -1),
        (-1, -1) => (-1, 1),
        (-1, 1) => (1, 1),
        _ => unreachable!(),
    };
    Position::new(x, y)
}

fn try_place_at(pos: Position, shape: &[Position], grid: &mut Present) -> bool {
    let empties = grid
        .relatives(pos, shape)
        .take_while(|(_, _, t)| **t == Tile::Empty)
        .map(|(_, p, _)| p)
        .collect::<Vec<_>>();

    if empties.len() == shape.len() {
        for pos in empties {
            *(grid.get_mut(pos).unwrap()) = Tile::Filled;
        }
        true
    } else {
        false
    }
}

fn undo_place(pos: Position, shape: &[Position], grid: &mut Present) {
    let positions = grid
        .relatives(pos, shape)
        .map(|(_, p, _)| p)
        .collect::<Vec<_>>();

    for pos in positions {
        *(grid.get_mut(pos).unwrap()) = Tile::Empty;
    }
}

fn solve(shapes: &Vec<[Vec<Position>; 4]>, grid: &mut Present, targets: &mut Vec<usize>) -> bool {
    let next_shape = targets
        .iter()
        .enumerate()
        .find(|(_, amnt)| **amnt != 0)
        .map(|(f, _)| f);

    let (x, y) = grid.size();

    if let Some(next_shape) = next_shape {
        let shape_all_rots = shapes.get(next_shape).unwrap();
        for y in 1..(y - 1) {
            for x in 1..(x - 1) {
                for shape in shape_all_rots {
                    if try_place_at(upos!(x, y), shape.as_slice(), grid) {
                        targets[next_shape] -= 1;
                        let inner_res = solve(shapes, grid, targets);
                        if inner_res {
                            return inner_res;
                        } else {
                            targets[next_shape] += 1;
                            undo_place(upos!(x, y), shape.as_slice(), grid);
                        }
                    }
                }
            }
        }
        false
    } else {
        true
    }
}

impl Day for Day12 {
    day_stuff!(12, "2", "🥳", Input);

    fn part_1((shapes, targets): Self::Input) -> Option<String> {
        let ans = targets
            .into_par_iter()
            .filter_map(|((x, y), mut avail)| {
                let area_needed = shapes
                    .iter()
                    .enumerate()
                    .map(|(i, s)| s[0].len() * avail[i])
                    .sum::<usize>();
                if area_needed > (x * y) {
                    return None;
                }
                let mut grid = Grid::new(vec![vec![Tile::Empty; x]; y]);
                let can_fit = solve(&shapes, &mut grid, &mut avail);
                if can_fit {
                    Some(0)
                } else {
                    None
                }
            })
            .count();

        Some(ans.to_string())
    }

    yippee!();

    fn parse_input(input: &str) -> Self::Input {
        let sections = input.split("\n\n").collect::<Vec<_>>();
        let shapes = sections
            .iter()
            .take(sections.len() - 1)
            .map(|s| {
                let (_, grid) = s.split_once('\n').unwrap();
                let grid = Present::parse(grid);
                let rels = grid
                    .relatives(Position::new(1, 1), &KERNS)
                    .filter(|(_, _, t)| **t == Tile::Filled)
                    .map(|(kern, _, _)| kern)
                    .collect::<Vec<_>>();

                let rels90: Vec<_> = rels.iter().copied().map(rot_pos).collect();

                let rels180: Vec<_> = rels90.iter().copied().map(rot_pos).collect();

                let rels270: Vec<_> = rels180.iter().copied().map(rot_pos).collect();

                [rels, rels90, rels180, rels270]
            })
            .collect();

        let targets = sections
            .last()
            .unwrap()
            .lines()
            .map(|l| {
                let (size, amnts) = l.split_once(':').unwrap();
                let (x, y) = size.split_once('x').unwrap();
                let amnts = amnts
                    .trim()
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect();
                ((x.parse().unwrap(), y.parse().unwrap()), amnts)
            })
            .collect();

        (shapes, targets)
    }
}
