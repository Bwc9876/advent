use std::collections::HashMap;

use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day11;

type Graph = HashMap<String, Vec<String>>;
type Seen<'a> = HashMap<&'a String, usize>;
type SeenConstrained<'a> = HashMap<(&'a String, bool, bool), usize>;

fn all_paths_to_out<'a>(node: &'a String, graph: &'a Graph, seen: &mut Seen<'a>) -> usize {
    if let Some(memo) = seen.get(&node) {
        *memo
    } else if let Some(nexts) = graph.get(node) {
        let mut amnt = 0;
        for next in nexts.iter() {
            if next == "out" {
                amnt += 1;
            } else {
                amnt += all_paths_to_out(next, graph, seen);
            }
        }
        seen.insert(node, amnt);
        amnt
    } else {
        0
    }
}

fn all_paths_to_out_constrained<'a>(
    node: &'a String,
    saw_dac: bool,
    saw_fft: bool,
    graph: &'a Graph,
    seen: &mut SeenConstrained<'a>,
) -> usize {
    if let Some(memo) = seen.get(&(node, saw_dac, saw_fft)) {
        *memo
    } else if let Some(nexts) = graph.get(node) {
        let mut amnt = 0;
        for next in nexts.iter() {
            if next == "out" {
                if saw_dac && saw_fft {
                    amnt += 1;
                }
            } else {
                amnt += all_paths_to_out_constrained(
                    next,
                    saw_dac || next == "dac",
                    saw_fft || next == "fft",
                    graph,
                    seen,
                );
            }
        }
        seen.insert((node, saw_dac, saw_fft), amnt);
        amnt
    } else {
        0
    }
}

impl Day for Day11 {
    day_stuff!(11, "5", "2", Graph);

    fn part_1(input: Self::Input) -> Option<String> {
        let mut seen = HashMap::with_capacity(input.len());
        let start = "you".to_string();
        let ans = all_paths_to_out(&start, &input, &mut seen);
        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let mut seen = HashMap::with_capacity(input.len());
        let start = "svr".to_string();
        let ans = all_paths_to_out_constrained(&start, false, false, &input, &mut seen);
        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (k, v) = l.split_once(':').unwrap();
                let v = v.trim().split(' ').map(str::to_string).collect::<Vec<_>>();
                (k.to_string(), v)
            })
            .collect()
    }
}
