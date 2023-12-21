use std::io::{stdin, Read};
use std::env::args;
#[derive(Clone, Debug)]
pub enum Selection {
    All,
    Single(usize), // TODO: Add range maybe?
}

impl Selection {

    fn parse(input: &str) -> Self {
        if input == "*" {
            Self::All
        } else {
            let input = input.parse::<usize>().unwrap();
            Self::Single(input)
        }
    }

}

#[derive(Clone, Debug)]
pub struct DP {
    pub day: Selection,
    pub part: Selection,
}

const DP_ALL: DP = DP {
    day: Selection::All,
    part: Selection::All,
};

impl DP {

    fn parse(input: &str) -> Self {
        let mut split = input.split(':');

        let day = split.next().map(Selection::parse).unwrap_or(Selection::All);
        let part = split.next().map(Selection::parse).unwrap_or(Selection::All);

        Self {
            day,
            part,
        }
    }

}

#[derive(Clone, Debug)]
pub struct YDP {
    pub year: Selection,
    pub day: Selection,
    pub part: Selection,
}

impl YDP {

    fn parse(input: &str) -> Self {
        let mut split = input.split(':');

        let year = split.next().map(Selection::parse).unwrap_or(Selection::All);
        let day = split.next().map(Selection::parse).unwrap_or(Selection::All);
        let part = split.next().map(Selection::parse).unwrap_or(Selection::All);

        Self {
            year,
            day,
            part,
        }
    }

    pub fn to_dp(&self) -> DP {
        DP {
            day: self.day.clone(),
            part: self.part.clone(),
        }
    }

}

pub fn get_dp_and_input() -> (DP, Option<String>) {
    let mut args = args().skip(1);

    let dp = args.next().map(|s| DP::parse(&s.trim())).unwrap_or(DP_ALL);

    let input = args.next().map(|s| s.trim().to_string()).map(|i| {
        if i == "-" {
            let mut input = String::new();
            stdin().read_to_string(&mut input).expect("Failed to read input");
            input.trim().to_string()
        } else {
            i
        }
    });

    (dp, input)
}

pub fn get_ydp_and_input(args: Vec<String>) -> (YDP, Option<String>) {

    let mut args = args.into_iter();

    let ydp = args.next().map(|s| YDP::parse(&s.trim())).unwrap_or(YDP {
        year: Selection::All,
        day: Selection::All,
        part: Selection::All,
    });

    let input = args.next().map(|s| s.trim().to_string()).map(|i| {
        if i == "-" {
            let mut input = String::new();
            stdin().read_to_string(&mut input).expect("Failed to read input");
            input.trim().to_string()
        } else {
            i
        }
    });

    (ydp, input)
}