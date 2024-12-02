
use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day2;

fn line_valid(line: &[u64]) -> bool {
    let mut increasing: Option<bool> = None;
    line.windows(2).all(|w| {
        let (x, y) = (w[0], w[1]);
        if x == y || x.abs_diff(y) > 3 {
            false
        } else if let Some(increasing) = increasing {
            !((increasing && y <= x) || (!increasing && x <= y))
        } else {
            increasing = Some(x < y);
            true
        }
    })
}

impl Day for Day2 {

    day_stuff!(2, "2", "4", Vec<Vec<u64>>);

    fn part_1(input: Self::Input) -> Option<String> {
        Some(input.into_iter().filter(|v| line_valid(&v)).count().to_string())
    }

    fn part_2(input: Self::Input) -> Option<String> {
        Some(input.into_iter().filter(|line| {
            let valid = line_valid(&line);
            if valid {
                true
            } else {
                (0..line.len()).any(|ie| { 
                    let v = line.iter()
                        .enumerate()
                        .filter(|(i, _)| *i != ie)
                        .map(|(_, e)| *e)
                        .collect::<Vec<_>>();
                    
                    line_valid(&v)
                })
            }
        }).count().to_string())
    }
    
    fn parse_input(input: &str) -> Self::Input {
        input.split('\n').map(|l| l.split_ascii_whitespace().map(|x| x.parse::<u64>().unwrap()).collect()).collect()
    }
}
