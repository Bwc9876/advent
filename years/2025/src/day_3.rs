use advent_core::{day_stuff, ex_for_day, Day};

pub struct Day3;

impl Day for Day3 {
    day_stuff!(3, "357", "3121910778619", Vec<Vec<usize>>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .map(|bank| {
                let (largest_idx, largest) = bank
                    .iter()
                    .enumerate()
                    .take(bank.len() - 1)
                    .rev()
                    .max_by(|(_, x), (_, y)| (**x).cmp(*y))
                    .map(|(i, x)| (i, *x))
                    .unwrap();
                let rest = bank.into_iter().skip(largest_idx + 1);
                let next_largest = rest.max().unwrap();
                largest * 10 + next_largest
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        let ans = input
            .into_iter()
            .map(|bank| {
                let mut num = 0;
                let mut start = 0;

                for i in 0..12 {
                    let (idx, largest) = bank
                        .iter()
                        .enumerate()
                        .filter(|(idx, _)| {
                            (i == 0 || *idx > start) && *idx + (11 - i) <= (bank.len() - 1)
                        })
                        .rev()
                        .max_by(|(_, x), (_, y)| (**x).cmp(*y))
                        .map(|(idx, x)| (idx, *x))
                        .unwrap();

                    num += largest * 10_usize.pow((11 - i) as u32);
                    start = idx;
                }

                num
            })
            .sum::<usize>();

        Some(ans.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                l.trim()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect()
    }
}
