#[macro_export]
macro_rules! yippee {
    () => {
        fn part_2(_: Self::Input) -> Option<String> {
            Some("🥳".to_string())
        }
    };
}

#[macro_export]
macro_rules! grid_day {
    ($day:literal, $e_1:literal, $e_2:literal, $t:ty) => {
        day_stuff!($day, $e_1, $e_2, utils::grid::Grid<$t>);

        fn parse_input(input: &str) -> Self::Input {
            Self::Input::parse(input)
        }
    };
}

#[macro_export]
macro_rules! lines_day {
    ($day:literal, $e_1:literal, $e_2:literal, $t:ty) => {
        day_stuff!($day, $e_1, $e_2, Vec<$t>);

        fn parse_input(input: &str) -> Self::Input {
            input.lines().map(|l| l.parse().unwrap()).collect()
        }
    };
}
