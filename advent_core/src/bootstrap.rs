use std::path::Path;

use regex::Regex;

use crate::MAX_DAY;

const DAY_TEMPLATE: &str = "
use advent_core::{Day, day_stuff, ex_for_day};

pub struct Day{day};

impl Day for Day{day} {

    day_stuff!({day}, \"\", \"\");

    fn part_1(_input: Self::Input) -> Option<String> {
        None
    }

    fn part_2(_input: Self::Input) -> Option<String> {
        None
    }
}
";

const YEAR_TEMPLATE: &str = "
use macros::year;

year!({year});
";

const RUNNER_TEMPLATE: &str = "
use macros::year_runner;

year_runner!({year});
";

const CARGO_TEMPLATE: &str = "
[package]
name = \"y_{year}\"
version = \"0.1.0\"
edition = \"2021\"

[dependencies]
advent_core = { path = \"../../advent_core\" }
macros = { path = \"../../macros\" }
utils = { path = \"../../utils\" }
";

fn make_day(folder: &Path, day: usize) {
    let day = day.to_string();

    let day_path = folder.join(format!("day_{}.rs", day));

    let contents = DAY_TEMPLATE.replace("{day}", &day);

    std::fs::write(day_path, contents).unwrap();
}

fn make_example(folder: &Path, day: usize) {
    let day = day.to_string();

    let example_path = folder.join(format!("day_{}", day));

    let part_1_path = example_path.join("1.txt");
    let part_2_path = example_path.join("2.txt");

    std::fs::create_dir_all(&example_path).unwrap();
    std::fs::write(part_1_path, "").unwrap();
    std::fs::write(part_2_path, "").unwrap();
}

fn make_days(folder: &Path) {
    for day in 1..=MAX_DAY {
        make_day(folder, day);
    }
}

fn make_examples(folder: &Path) {
    let examples_path = folder.join("examples");
    for day in 1..=MAX_DAY {
        make_example(&examples_path, day);
    }
}

fn make_lib(folder: &Path, year: &str) {
    let lib_path = folder.join("lib.rs");

    let contents = YEAR_TEMPLATE.replace("{year}", year);

    std::fs::write(lib_path, contents).unwrap();
}

fn make_main(folder: &Path, year: &str) {
    let main_path = folder.join("main.rs");

    let contents = RUNNER_TEMPLATE.replace("{year}", year);

    std::fs::write(main_path, contents).unwrap();
}

fn make_src(folder: &Path, year: &str) {
    let src_path = folder.join("src");

    std::fs::create_dir_all(&src_path).unwrap();

    make_days(&src_path);
    make_examples(&src_path);
    make_lib(&src_path, year);
    make_main(&src_path, year);
}

fn make_cargo(folder: &Path, year: &str) {
    let cargo_path = folder.join("Cargo.toml");

    let contents = CARGO_TEMPLATE.replace("{year}", year);

    std::fs::write(cargo_path, contents).unwrap();
}

fn replace_year_list(new_year: &str) {
    let main = include_str!("../../src/main.rs");

    let global_runner_pattern = Regex::new(r"global_runner!\(([\d,]+)\)").unwrap();

    let matches = global_runner_pattern.captures(main).unwrap();

    let full = matches.get(0).unwrap().as_str();

    let mut years = matches
        .get(1)
        .unwrap()
        .as_str()
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    years.push(new_year.parse::<usize>().unwrap());

    years.sort();

    let new_years = years
        .iter()
        .map(|y| y.to_string())
        .collect::<Vec<_>>()
        .join(",");

    let new_main = main.replace(full, &format!("global_runner!({})", new_years));

    std::fs::write("src/main.rs", new_main).unwrap();
}

fn replace_cargo_dependencies(new_year: &str) {
    let cargo = include_str!("../../Cargo.toml");

    let new_dep = format!("y_{year} = {{ path = \"years/{year}\" }}", year = new_year);

    let cargo = cargo.replace("[dependencies]", &format!("[dependencies]\n{}", new_dep));

    std::fs::write("Cargo.toml", cargo).unwrap();
}

pub fn make_year(year: &str) {
    let cwd = std::env::current_dir().unwrap();

    let year_path = cwd.join(format!("years/{}", year));

    std::fs::create_dir_all(&year_path).unwrap();

    make_src(&year_path, year);
    make_cargo(&year_path, year);

    replace_cargo_dependencies(year);
    replace_year_list(year);
}
