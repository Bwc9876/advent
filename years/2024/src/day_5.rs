
use std::collections::{HashMap, HashSet};

use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day5;

#[derive(Clone, Debug)]
pub struct Rules(HashMap<i64,Vec<i64>>);

impl Rules {
    pub fn take(&mut self, lhs: i64, rhs: i64) {
        self.0.entry(lhs).and_modify(|seen| { seen.push(rhs); }).or_insert(vec![rhs]);
    }

    fn good(&self, up: &[i64]) -> bool {
        let mut seen = HashSet::new();
        for u in up.iter() {
            if self.0.get(u).is_none_or(|should| should.iter().all(|s| !seen.contains(s))) {
                seen.insert(*u);  
            } else {
                return false;
            }
        }
        true
    }

    fn sort_to_rules(&self, up: &mut[i64]) {
        while !self.good(up) {
            self.sort_step(up);
        }
    }

    fn sort_step(&self, up: &mut[i64]) {
        for i in 0..up.len()-1 {
            for j in i+1..up.len() {
                if self.0.get(&up[j]).is_some_and(|shld| shld.contains(&up[i])) {
                    up.swap(i, j);
                } 
            }
        }
    }
}

impl Day for Day5 {

    day_stuff!(5, "143", "123", (Rules, Vec<Vec<i64>>));

    fn part_1(input: Self::Input) -> Option<String> {
        let (rules, updates) = input;
        let ans = updates.into_iter().filter_map(|up| {
            if rules.good(&up) {
                Some(up[up.len() / 2])
            } else {
                None
            }
        }).sum::<i64>();
        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let (rules, updates) = input;
        let ans = updates.into_iter().filter_map(|mut up| {
            if !rules.good(&up) {
                rules.sort_to_rules(&mut up);
                Some(up[up.len() / 2])
            } else {
                None
            }
        }).sum::<i64>();
        Some(ans.to_string())
    }
    
    fn parse_input(input: &str) -> Self::Input {
        let (rules, updates) = input.split_once("\n\n").unwrap();
        let mut rules_o = Rules(HashMap::with_capacity(50));
        for l in rules.lines() {
            let (lhs, rhs) = l.split_once('|').unwrap();
            rules_o.take(lhs.parse().unwrap(), rhs.parse().unwrap());
        }
        (rules_o, updates.split('\n').map(|l| l.split(',').map(|x| x.parse().unwrap()).collect()).collect())
    }
}
