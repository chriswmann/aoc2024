use aoc2024::helpers::load_data;
use aoc2024::run::run;
use clap::Parser;

fn main() {
    tracing_subscriber::fmt::init();
    let args = aoc2024::cli::Cli::parse();
    match args.day {
        d @ 1..26 => {
            let data = load_data(d);
            run(&data, d, args.part);
        }
        _ => eprintln!("Invalid day: {}. Must be in range 1..=25.", args.day),
    }
}
