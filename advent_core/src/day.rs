use std::time::Instant;

use indicatif::ProgressStyle;

#[macro_export]
macro_rules! ex_for_day {
    ($day:literal, $part:literal) => {
        include_str!(concat!(
            "examples/day_",
            stringify!($day),
            "/",
            stringify!($part),
            ".txt"
        ))
    };
}

#[macro_export]
macro_rules! day_stuff {
    ($day:literal, $e_1:literal, $e_2:literal) => {
        day_stuff!($day, $e_1, $e_2, String);

        fn parse_input(input: &str) -> Self::Input {
            input.to_string()
        }
    };

    ($day:literal, $e_1:literal, $e_2:literal, $i: ty) => {
        type Input = $i;

        const DAY: usize = $day;
        const EXAMPLE_INPUT_1: &'static str = ex_for_day!($day, 1);
        const EXAMPLE_INPUT_2: &'static str = ex_for_day!($day, 2);
        const EXPECTED_1: &'static str = $e_1;
        const EXPECTED_2: &'static str = $e_2;
    }
}

const BENCH_SECS: u64 = 5;
const PROGRESS_TEMPLATE: &str = "{spinner} {wide_msg} [{bar:100.green/cyan}]";
const PROGRESS_CHARS: &str = "=>-";

macro_rules! bench {
    ($t:expr) => {{
        let outer_instant = std::time::Instant::now();
        let mut times = Vec::<std::time::Duration>::with_capacity(1000);
        let mut res = None;
        let style = ProgressStyle::with_template(PROGRESS_TEMPLATE)
            .unwrap()
            .progress_chars(PROGRESS_CHARS);
        let progress = indicatif::ProgressBar::new(BENCH_SECS);
        progress.set_style(style);
        let mut i = 0;
        while outer_instant.elapsed().as_secs() <= BENCH_SECS {
            let instant = std::time::Instant::now();
            let r = $t;
            if res.is_none() {
                res = Some(r)
            }
            times.push(instant.elapsed());
            i += 1;
            progress.set_message(format!("{i} Runs"));
            progress.set_position(outer_instant.elapsed().as_secs());
        }
        (
            times.iter().sum::<std::time::Duration>() / (times.len() as u32),
            times.len(),
            res.unwrap(),
        )
    }};
}

/// A trait for a day of Advent of Code.
///
/// This trait is implemented for each day of Advent of Code.
/// You're expected to implement the `EXAMPLE_INPUT_1` and `EXAMPLE_INPUT_2` constants
/// with the example inputs for each part of the day.
///
/// You're also expected to implement the `EXPECTED_1` and `EXPECTED_2` constants
/// with the expected outputs for each part of the day.
///
/// Then, any runner can use `run_part` to run a part of the day with a given input or the example input.
///
pub trait Day {
    type Input: Clone;

    const DAY: usize = 0;

    const EXAMPLE_INPUT_1: &'static str = "";
    const EXAMPLE_INPUT_2: &'static str = "";

    const EXPECTED_1: &'static str = "";
    const EXPECTED_2: &'static str = "";

    fn get_example_input(part: usize) -> &'static str {
        match part {
            1 => Self::EXAMPLE_INPUT_1,
            2 => Self::EXAMPLE_INPUT_2,
            _ => panic!("Invalid part number"),
        }
    }

    fn run_part(part: usize, input: Option<&str>) -> Option<String> {
        let input = input.unwrap_or_else(|| Self::get_example_input(part));
        let input = Self::parse_input(input);
        let instant = Instant::now();
        let solution = match part {
            1 => Self::part_1(input),
            2 => Self::part_2(input),
            _ => panic!("Invalid part number"),
        };
        println!(
            "Day {} Part {}: {} ({:?}ms)",
            Self::DAY,
            part,
            solution.as_ref().unwrap_or(&"Not implemented".to_string()),
            instant.elapsed()
        );
        solution
    }

    fn bench_part(part: usize, input: Option<&str>) {
        let input = input.unwrap_or_else(|| Self::get_example_input(part));
        let (parse_time, sample_size, input) = bench!(Self::parse_input(input));
        println!(
            "Day {} Parse Func: {:?} (N = {})",
            Self::DAY,
            parse_time,
            sample_size
        );

        let (part_time, sample_size, _output) = match part {
            1 => bench!(Self::part_1(input.clone())),
            2 => bench!(Self::part_2(input.clone())),
            _ => panic!("Invalid Part Number"),
        };

        println!(
            "Day {} Part {}: {:?} (N = {})",
            Self::DAY,
            part,
            part_time,
            sample_size
        );
    }

    fn run_all_parts(extra_indent: &str) {
        println!(
            "{extra_indent}Day {day}:",
            extra_indent = extra_indent,
            day = Self::DAY
        );
        for part in 1..=2 {
            let part_time = Instant::now();
            let solution = match part {
                1 => Self::part_1(Self::parse_input(Self::EXAMPLE_INPUT_1)),
                2 => Self::part_2(Self::parse_input(Self::EXAMPLE_INPUT_2)),
                _ => panic!("Invalid part number"),
            };
            println!(
                "{extra_indent}  Part {}: {} ({:?}ms)",
                part,
                solution.as_ref().unwrap_or(&"Not implemented".to_string()),
                part_time.elapsed()
            );
        }
    }

    fn parse_input(input: &str) -> Self::Input;

    fn part_1(_input: Self::Input) -> Option<String> {
        None
    }
    fn part_2(_input: Self::Input) -> Option<String> {
        None
    }

    fn assert_part_1() {
        let expected = Self::EXPECTED_1;
        let actual = Self::run_part(1, None);
        if let Some(actual) = actual {
            assert_eq!(actual, expected);
        }
    }

    fn assert_part_2() {
        let expected = Self::EXPECTED_2;
        let actual = Self::run_part(2, None);
        if let Some(actual) = actual {
            assert_eq!(actual, expected);
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    struct TestDay;

    impl Day for TestDay {
        type Input = String;

        const EXAMPLE_INPUT_1: &'static str = "Hello, world!";
        const EXAMPLE_INPUT_2: &'static str = "Hello, world!";

        const EXPECTED_1: &'static str = "Goodbye, world!";
        const EXPECTED_2: &'static str = "Hello, moon!";

        fn parse_input(input: &str) -> String {
            input.to_string()
        }

        fn part_1(input: String) -> Option<String> {
            Some(input.replace("Hello", "Goodbye"))
        }

        fn part_2(input: String) -> Option<String> {
            Some(input.replace("world", "moon"))
        }
    }

    struct TestDay2;

    impl Day for TestDay2 {
        type Input = Vec<String>;

        const EXAMPLE_INPUT_1: &'static str = "A\nB\nC";

        const EXPECTED_1: &'static str = "A,B,C";

        fn parse_input(input: &str) -> Vec<String> {
            input.lines().map(|l| l.to_string()).collect::<Vec<_>>()
        }

        fn part_1(input: Vec<String>) -> Option<String> {
            Some(input.join(",").to_string())
        }
    }

    #[test]
    fn test_day_1() {
        TestDay::assert_part_1();
        TestDay::assert_part_2();
    }

    #[test]
    fn test_day_2() {
        TestDay2::assert_part_1();
        // Should skip
        TestDay2::assert_part_2();
    }
}
