use advent_core::{Day, day_stuff, ex_for_day};
use utils::range::BetterRange;

pub struct Day5;

impl Day for Day5 {
    day_stuff!(5, "3", "14", (Vec<BetterRange<usize>>, Vec<usize>));

    fn part_1((ranges, ids): Self::Input) -> Option<String> {
        let ans = ids
            .into_iter()
            .filter(|id| ranges.iter().any(|range| range.contains(id)))
            .count();

        Some(ans.to_string())
    }

    fn part_2((mut ranges, _): Self::Input) -> Option<String> {
        ranges.sort_by(|a, b| a.start.cmp(&b.start).then(a.end.cmp(&b.end)));

        let mut new_ranges = Vec::with_capacity(ranges.len());

        for range in ranges {
            let mut merged = false;

            for new_range in new_ranges.iter_mut() {
                if let Some(merged_range) = range.merge(new_range) {
                    merged = true;
                    *new_range = merged_range;
                    break;
                }
            }

            if !merged {
                new_ranges.push(range);
            }
        }

        let ans = new_ranges
            .into_iter()
            .map(|range| range.end - range.start)
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        let (ranges, ids) = input.split_once("\n\n").unwrap();

        (
            ranges
                .lines()
                .map(|l| {
                    let (lower, upper) = l.split_once('-').unwrap();
                    BetterRange::new(lower.parse().unwrap(), upper.parse::<usize>().unwrap() + 1)
                })
                .collect(),
            ids.lines().map(|l| l.parse().unwrap()).collect(),
        )
    }
}
