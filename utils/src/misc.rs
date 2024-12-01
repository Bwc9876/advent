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
