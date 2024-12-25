use std::collections::{HashMap, VecDeque};

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{
    dir::{Direction, CARDINALS},
    pos::Position,
};

pub struct Day21;

const NUMPAD: &str = "789\n456\n123\n#0A";
const DIRPAD: &str = "#^A\n<v>";

type Grid = utils::grid::Grid<char>;

fn pad_grids() -> (Grid, Grid) {
    (Grid::parse(NUMPAD), Grid::parse(DIRPAD))
}

fn dir_to_char(dir: Direction) -> char {
    match dir {
        Direction::East => '>',
        Direction::West => '<',
        Direction::North => '^',
        Direction::South => 'v',
    }
}

type BestMap = HashMap<(char, char), Vec<Vec<char>>>;

fn find_best_paths(g: &Grid) -> BestMap {
    let mut costs = BestMap::with_capacity(18);

    for (pos1, tile1) in g.iter().filter(|(_, t)| **t != '#') {
        costs.insert((*tile1, *tile1), vec![vec![]]);
        for (pos2, tile2) in g.iter().filter(|(_, t)| **t != '#' && **t != *tile1) {
            let mut queue = VecDeque::<(Position, Vec<char>)>::with_capacity(18);
            queue.push_back((pos1, vec![]));
            while let Some((curr_pos, path)) = queue.pop_front() {
                if costs
                    .get(&(*tile1, *tile2))
                    .is_some_and(|c| c[0].len() < path.len())
                {
                    continue;
                }

                if curr_pos == pos2 {
                    (*costs.entry((*tile1, *tile2)).or_insert(vec![])).push(path);
                    continue;
                }

                for (dir, new_pos, _) in g
                    .relatives(curr_pos, &CARDINALS)
                    .filter(|(_, _, t)| **t != '#' && **t != *tile1)
                {
                    let mut new_path = path.clone();
                    new_path.push(dir_to_char(dir));
                    queue.push_back((new_pos, new_path));
                }
            }
        }
    }

    costs
        .values_mut()
        .for_each(|v| v.iter_mut().for_each(|p| p.push('A')));

    costs
}

fn recur_find(
    seq: &[char],
    level: usize,
    top: bool,
    robos: &mut Vec<char>,
    num_best: &BestMap,
    dir_best: &BestMap,
    dp: &mut HashMap<(Vec<char>, usize, char), usize>,
) -> usize {
    let key = (seq.to_vec(), level, robos[level]);
    if let Some(&res) = dp.get(&key) {
        return res;
    }

    let mut final_val = 0;

    for &c in seq {
        let all_paths = (if top { num_best } else { dir_best })
            .get(&(robos[level], c))
            .unwrap();
        final_val += if level == 0 {
            all_paths.iter().map(Vec::len).min()
        } else {
            all_paths
                .iter()
                .map(|path| recur_find(path, level - 1, false, robos, num_best, dir_best, dp))
                .min()
        }
        .unwrap();
        robos[level] = c;
    }

    dp.insert(key, final_val);

    final_val
}

impl Day for Day21 {
    day_stuff!(21, "126384", "154115708116294", Vec<(usize, Vec<char>)>);

    fn part_1(input: Self::Input) -> Option<String> {
        let (num_grid, dir_grid) = pad_grids();
        let (num_best, dir_best) = (find_best_paths(&num_grid), find_best_paths(&dir_grid));
        let mut dp = HashMap::new();
        let ans = input
            .into_iter()
            .map(|(num, code)| {
                let mut robos = vec!['A'; 3];
                let best_path =
                    recur_find(&code, 2, true, &mut robos, &num_best, &dir_best, &mut dp);

                best_path * num
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (num_grid, dir_grid) = pad_grids();
        let (num_best, dir_best) = (find_best_paths(&num_grid), find_best_paths(&dir_grid));
        let mut dp = HashMap::new();
        let ans = input
            .into_iter()
            .map(|(num, code)| {
                let mut robos = vec!['A'; 26];
                let best_path =
                    recur_find(&code, 25, true, &mut robos, &num_best, &dir_best, &mut dp);

                best_path * num
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|l| {
                (
                    l.trim_start_matches('0')
                        .trim_end_matches('A')
                        .parse::<usize>()
                        .unwrap(),
                    l.chars().collect(),
                )
            })
            .collect()
    }
}
