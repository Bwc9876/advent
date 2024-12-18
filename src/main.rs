use advent_core::{get_ydp_and_input, make_year, Selection, Year, DP, YDP};
use macros::global_runner;

global_runner!(2024);

fn run_ydp(ydp: YDP, input: Option<String>) {
    let dp = ydp.to_dp();

    match ydp.year {
        Selection::All => {
            run_all_years(&dp, input);
        }
        Selection::Single(year) => {
            run_year(year, dp, input.as_deref());
        }
    }
}

fn bench_ydp(ydp: YDP, input: Option<String>) {
    let dp = ydp.to_dp();

    match ydp.year {
        Selection::Single(year) => {
            bench_year(year, dp, input.as_deref());
        }
        _ => panic!("Cannot Bench All Years"),
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();

    let command = args.first();

    match command {
        Some(command) => match command.as_str() {
            "new" => {
                let year = args.get(1).expect("No year provided");
                make_year(year);
            }
            "solve" | "run" => {
                let (ydp, input) = get_ydp_and_input(args[1..].to_vec());
                run_ydp(ydp, input);
            }
            "bench" => {
                let (ydp, input) = get_ydp_and_input(args[1..].to_vec());
                bench_ydp(ydp, input);
            }
            _ => {
                println!("Unknown command: {}", command);
                println!("Available commands: new, solve, bench");
            }
        },
        None => {
            println!("No command provided");
            println!("Available commands: new, solve");
        }
    }
}
