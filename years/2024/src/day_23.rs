use std::collections::{HashMap, HashSet};

use advent_core::{day_stuff, ex_for_day, Day};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub struct Day23;

type Edges = HashMap<String, HashSet<String>>;

fn find_self_in_3<'a>(node: &'a String, edges: &'a Edges) -> Vec<[&'a String; 3]> {
    edges
        .get(node)
        .unwrap()
        .iter()
        .flat_map(|adj| {
            edges.get(adj).unwrap().iter().filter_map(|sadj| {
                if edges.get(sadj).unwrap().contains(node) {
                    let mut v = [node, adj, sadj];
                    v.sort();
                    Some(v)
                } else {
                    None
                }
            })
        })
        .collect()
}

fn represent<'a>(
    node: &'a String,
    friends: HashSet<&'a String>,
    edges: &'a Edges,
    size: usize,
    seen: &mut HashSet<&'a String>,
) -> Option<HashSet<&'a String>> {
    if friends.len() == size {
        return Some(friends);
    } else if friends.iter().all(|f| seen.contains(f)) {
        return None;
    }

    seen.insert(node);

    edges.get(node).unwrap().iter().find_map(|dep| {
        if edges
            .get(dep)
            .is_some_and(|e| friends.iter().all(|f| e.contains(*f)))
        {
            let mut new_friends = friends.clone();
            new_friends.insert(dep);
            represent(dep, new_friends, edges, size, seen)
        } else {
            None
        }
    })
}

impl Day for Day23 {
    day_stuff!(23, "7", "co,de,ka,ta", Edges);

    fn part_1(input: Self::Input) -> Option<String> {
        let groups = input
            .keys()
            .flat_map(|k| find_self_in_3(k, &input))
            .collect::<HashSet<_>>();

        let ans = groups
            .into_iter()
            .filter(|g| g.iter().any(|c| c.starts_with('t')))
            .count();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let verts = input.keys().cloned().collect::<Vec<_>>();
        let max_group_size = verts
            .iter()
            .map(|v| input.get(v).unwrap().len())
            .max()
            .unwrap()
            + 1;

        let mut group = (0..max_group_size)
            .rev()
            .find_map(|s| {
                verts.par_iter().find_map_any(|node| {
                    represent(
                        node,
                        HashSet::from_iter([node]),
                        &input,
                        s,
                        &mut HashSet::with_capacity(s),
                    )
                })
            })
            .unwrap()
            .into_iter()
            .collect::<Vec<_>>();

        group.sort();
        let len = group.len();
        let s = ",".to_string();
        Some(group.into_iter().intersperse(&s).fold(
            String::with_capacity(len * 2 + (len - 1)),
            |mut acc, computer| {
                acc.push_str(computer.as_str());
                acc
            },
        ))
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .fold(HashMap::with_capacity(50), |mut acc, l| {
                let (l, r) = l.split_once('-').unwrap();
                acc.entry(l.to_string())
                    .or_insert(HashSet::new())
                    .insert(r.to_string());
                acc.entry(r.to_string())
                    .or_insert(HashSet::new())
                    .insert(l.to_string());
                acc
            })
    }
}
