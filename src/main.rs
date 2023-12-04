use clap::Parser;

use aoc_2015_rust::Selector;

use crate::solutions::run;

mod solutions;

/// AoC 2015
#[derive(Parser, Debug)]
#[clap(
    author = "Mikko Leppänen <mleppan23@gmail.com>",
    version = "0.1",
    about
)]
pub struct Args {
    /// Execute all solutions
    #[arg(short, long, default_value_t = false)]
    pub all: bool,
    /// Execute particular day between 1..25
    #[arg(short, long, value_delimiter = ' ', num_args=0..=2)]
    pub day: Option<Vec<u8>>,
    /// Execute last solution
    #[arg(short, long, default_value_t = true)]
    pub last: bool,
}
fn main() {
    let args = Args::parse();

    if let Some(day) = args.day {
        if !day.is_empty() && matches!(day[0], 1..=25) {
            run(Selector::One(day));
            return;
        }
    }

    if args.all {
        run(Selector::All);
        return;
    }

    if args.last {
        run(Selector::Last);
    }
}
