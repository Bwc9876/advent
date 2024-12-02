use core::ops::Range;
use std::{collections::HashMap, hash::Hash};

pub fn counts<T: Hash + Eq + PartialEq>(l: impl Iterator<Item = T>) -> HashMap<T, u64> {
    let (min, max) = l.size_hint();
    l.fold(
        HashMap::with_capacity(max.unwrap_or(min)),
        |mut agg, curr| {
            agg.entry(curr).and_modify(|x| *x += 1).or_insert(1);
            agg
        },
    )
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum FollowRangeResult {
    InvalidPair(i64, i64),
    Increasing,
    Decreasing,
    Fluctuates,
    ListTooShort
}

pub fn follows_diff_range(l: &[i64], diff_range: Range<i64>, enforce_one_way: bool, allow_eq: bool) -> FollowRangeResult {
    if l.len() < 2 {
        FollowRangeResult::ListTooShort
    } else {
        let first_diff = l[1] - l[0];
        if diff_range.contains(&first_diff) {
            let mut ordering = FollowRangeResult::Fluctuates;
            
            let failing_pair = l.windows(2).skip(1).find_map(|w| {
                let (x, y) = (w[0], w[1]);
                let diff = y - x;
                if diff_range.contains(&diff) && (allow_eq || diff != 0) {
                    if ordering == FollowRangeResult::Fluctuates && diff != 0 {
                        ordering = if diff < 0 { FollowRangeResult::Decreasing } else { FollowRangeResult::Increasing };
                        None
                    } else if enforce_one_way && (ordering == FollowRangeResult::Increasing && diff < 0) || (ordering == FollowRangeResult::Decreasing && diff > 0) {
                        Some((x, y))
                    } else {
                        None
                    }
                } else {
                    Some((x, y))
                }
            });

            match failing_pair {
                Some((x, y)) => FollowRangeResult::InvalidPair(x, y),
                None => ordering
            }
        } else {
            FollowRangeResult::InvalidPair(l[0], l[1])
        }
    }
}

pub fn all_combos_remove_one<T>(l: &[T]) -> impl Iterator<Item = impl Iterator<Item = &T>> {
    (0..l.len()).map(|exclude| {
        l.iter().enumerate().filter_map(move |(i, e)| Some(e).filter(|_| i != exclude))
    })
}
