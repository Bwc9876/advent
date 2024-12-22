use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use advent_core::{day_stuff, ex_for_day, Day};
use utils::{ipos, pos::Position, upos};

pub struct Day18;

#[derive(Clone, Eq, PartialEq)]
struct DState {
    cost: usize,
    pos: Position,
    step_no: usize,
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

impl Day for Day18 {
    day_stuff!(18, "", "", Vec<Position>);

    fn part_1(input: Self::Input) -> Option<String> {
        let start_pos = Position::zero();
        let end_pos = ipos!(70, 70);

        let mut queue = BinaryHeap::new();

        queue.push(DState {
            cost: 0,
            pos: start_pos,
            step_no: 0,
        });

        let mut dist = HashMap::<Position, usize>::new();

        while let Some(DState { cost, pos, step_no }) = queue.pop() {
            if pos == end_pos {
                return Some(cost.to_string());
            }

            if dist.get(&pos).is_some_and(|min_score| *min_score < cost) {
                continue;
            }

            for (next_pos, _dir) in pos
                .adjacents_checked((71, 71))
                .filter(|(p, _)| input.iter().take(1024).all(|op| op != p))
            {
                let next_state = DState {
                    cost: cost + 1,
                    pos: next_pos,
                    step_no: step_no + 1,
                };
                if next_state.cost < *dist.get(&next_state.pos).unwrap_or(&usize::MAX) {
                    *dist.entry(next_state.pos).or_insert(usize::MAX) = next_state.cost;
                    queue.push(next_state);
                }
            }
        }

        panic!("No Path")
    }

    fn part_2(input: Self::Input) -> Option<String> {
        for i in 0..input.len() {
            println!("Byte {} of {}", i + 1, input.len());
            let start_pos = Position::zero();
            let end_pos = ipos!(70, 70);

            let mut queue = BinaryHeap::new();

            queue.push(DState {
                cost: 0,
                pos: start_pos,
                step_no: 0,
            });

            let mut dist = HashMap::<Position, usize>::new();
            let mut flag = false;

            while let Some(DState { cost, pos, step_no }) = queue.pop() {
                if pos == end_pos {
                    flag = true;
                    break;
                }

                if dist.get(&pos).is_some_and(|min_score| *min_score < cost) {
                    continue;
                }

                for (next_pos, _dir) in pos
                    .adjacents_checked((71, 71))
                    .filter(|(p, _)| input.iter().take(i + 1).all(|op| op != p))
                {
                    let next_state = DState {
                        cost: cost + 1,
                        pos: next_pos,
                        step_no: step_no + 1,
                    };
                    if next_state.cost < *dist.get(&next_state.pos).unwrap_or(&usize::MAX) {
                        *dist.entry(next_state.pos).or_insert(usize::MAX) = next_state.cost;
                        queue.push(next_state);
                    }
                }
            }
            if !flag {
                return Some(input[i].to_string());
            }
        }
        None
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .trim()
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                upos!(x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
            })
            .collect()
    }
}
