use advent_core::{day_stuff, ex_for_day, Day};
use utils::pos::Position;

pub struct Day9;

impl Day for Day9 {
    day_stuff!(9, "50", "24", Vec<Position>);

    fn part_1(input: Self::Input) -> Option<String> {
        let ans = input
            .iter()
            .enumerate()
            .flat_map(|(i, a)| {
                input.iter().skip(i + 1).map(|b| {
                    let p = (*b - *a).abs();
                    (p.x + 1) * (p.y + 1)
                })
            })
            .max()
            .unwrap();

        Some(ans.to_string())
    }

    fn part_2(mut input: Self::Input) -> Option<String> {
        let mut max = 0;

        input.push(*input.first().unwrap());

        let lines = input.windows(2).map(|w| (w[0], w[1])).collect::<Vec<_>>();

        for (i, a) in input.iter().enumerate() {
            // println!("Start {} ({}/{})", *a, i + 1, input.len());
            for b in input.iter().skip(i + 1) {
                let p = (*b - *a).abs();
                let area = (p.x + 1) * (p.y + 1);

                if area > max {
                    let check = lines.iter().any(|&l| {
                        let (cons, b1, b2, c1, c2, r1, r2) = if l.0.x == l.1.x {
                            (l.0.x, a.x, b.x, l.0.y, l.1.y, a.y, b.y)
                        } else {
                            (l.0.y, a.y, b.y, l.0.x, l.1.x, a.x, b.x)
                        };

                        let (lower_cons, upper_cons) = (b1.min(b2), b1.max(b2));
                        let (line_range_lower, line_range_upper) = (c1.min(c2), c1.max(c2));
                        let (rect_range_lower, rect_range_upper) = (r1.min(r2), r1.max(r2));

                        let cons_in_range = cons > lower_cons && cons < upper_cons;
                        let ranges_overlap = if line_range_lower < rect_range_lower {
                            line_range_upper > rect_range_lower
                        } else {
                            rect_range_upper > line_range_lower
                        };

                        cons_in_range && ranges_overlap
                    });

                    if !check {
                        max = area;
                    }
                }
            }
        }

        Some(max.to_string())
    }

    fn parse_input(input: &str) -> Self::Input {
        input
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                Position {
                    x: x.parse().unwrap(),
                    y: y.parse().unwrap(),
                }
            })
            .collect()
    }
}
