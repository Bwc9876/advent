mod bootstrap;
mod day;
mod parser;
mod year;

pub const MAX_DAY: usize = 25;

pub use bootstrap::make_year;
pub use day::Day;
pub use parser::{get_dp_and_input, get_ydp_and_input, Selection, DP, YDP};
pub use year::Year;
