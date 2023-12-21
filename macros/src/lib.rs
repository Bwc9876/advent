extern crate proc_macro;

use core::MAX_DAY;

use proc_macro::TokenStream;

fn make_day_mods() -> String {
    (1..=MAX_DAY).map(|day| format!("mod day_{day};", day = day)).collect::<Vec<_>>().join("\n")
}

fn make_use_days() -> String {
    (1..=MAX_DAY).map(|day| format!("use day_{day}::Day{day};", day = day)).collect::<Vec<_>>().join("\n")
}

fn make_day_match(inner: &str) -> String {
    (1..=MAX_DAY).map(|day| format!("{day} => {},", inner.replace("{day}", &day.to_string()))).collect::<Vec<_>>().join("\n")
}

fn make_day_tests() -> String {
    (1..=MAX_DAY).map(|day| format!("
    #[test]
    fn test_day_{day}_part_1() {{
        Day{day}::assert_part_1();
    }}

    #[test]
    fn test_day_{day}_part_2() {{
        Day{day}::assert_part_2();
    }}")).collect::<Vec<_>>().join("\n")
}

fn get_solve_day() -> String {
    let inner = make_day_match("Day{day}::run_part(part, input)");
    format!("
    fn solve_day(day: usize, part: usize, input: Option<&str>) -> Option<String> {{
        match day {{
            {inner}
            _ => None,
        }}
    }}", inner = inner)
}

fn get_solve_day_both_parts() -> String {
    let inner = make_day_match("Day{day}::run_all_parts(extra_indent)");
    format!("
    fn solve_day_both_parts(day: usize, extra_indent: &str) {{
        match day {{
            {inner}
            _ => (),
        }}
    }}", inner = inner)
}

fn make_year_struct(year: &str) -> String {
    format!("
        pub struct Year{year};

        impl Year for Year{year} {{
            const YEAR: usize = {year};

            {solve_day}

            {solve_day_both_parts}
        }}", solve_day = get_solve_day(), solve_day_both_parts = get_solve_day_both_parts())
}

fn make_tests() -> String {
    format!("
    #[cfg(test)]
    mod tests {{
        use super::*;
        use core::{{Day, Year}};

        {day_tests}
    }}", day_tests = make_day_tests())
}

#[proc_macro]
pub fn year(item: TokenStream) -> TokenStream {
    let year = item.to_string();

    let mods = make_day_mods();
    let uses = make_use_days();

    let year_struct = make_year_struct(&year);

    let tests = make_tests();

    format!("
        {mods}

        use core::{{Year, Day}};
        {uses}

        {year_struct}

        {tests}
    ").parse::<TokenStream>().unwrap()
}

#[proc_macro]
pub fn year_runner(item: TokenStream) -> TokenStream {
    let year = item.to_string();

    format!("
    use core::{{Year, get_dp_and_input}};

    use y_{year}::Year{year};

    fn main() {{
        let (dp, input) = get_dp_and_input();
        Year{year}::run_dp(input.as_deref(), dp);
    }}").parse::<TokenStream>().unwrap()
}

fn make_year_match(years: &Vec<&str>, inner: &str) -> String {
    years.iter().map(|year| format!("{year} => {},", inner.replace("{year}", &year.to_string()))).collect::<Vec<_>>().join("\n")
}

fn make_year_uses(years: &Vec<&str>) -> String {
    years.iter().map(|year| format!("use y_{year}::Year{year};", year = year)).collect::<Vec<_>>().join("\n")
}

fn make_run_all_years(years: &Vec<&str>) -> String {
    years.iter().map(|year| format!("Year{year}::run_dp(input.as_deref(), dp.clone());", year = year)).collect::<Vec<_>>().join("\n")
}

fn make_run_year(years: &Vec<&str>) -> String {
    let inner = make_year_match(years, "Year{year}::run_dp(input.as_deref(), dp)");
    format!("
    fn run_year(year: usize, dp: DP, input: Option<&str>) {{
        match year {{
            {inner}
            _ => {{
                println!(\"Unknown year: {{year}}\");
            }}
        }}
    }}", inner = inner)
}

#[proc_macro]
pub fn global_runner(item: TokenStream) -> TokenStream {
    let item = item.to_string();
    let years = item.split(',').map(|s| s.trim()).collect::<Vec<_>>();

    let year_uses = make_year_uses(&years);
    let run_all_years = make_run_all_years(&years);
    let run_year = make_run_year(&years);

    format!("
    {year_uses}

    {run_year}

    fn run_all_years(dp: &DP, input: Option<String>) {{
        {run_all_years}
    }}").parse::<TokenStream>().unwrap()
}