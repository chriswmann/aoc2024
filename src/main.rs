use aoc2024::day01::part01;
use aoc2024::helpers::load_data;
use clap::Parser;

fn main() {
    tracing_subscriber::fmt::init();
    let args = aoc2024::cli::Cli::parse();
    match args.day {
        1 => match args.part {
            1 => {
                let data = load_data(1, 1);
                part01(&data);
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    }
}
