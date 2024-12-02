#[derive(clap::Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(long, value_parser=clap::value_parser!(u8).range(1..=25),)]
    pub day: u8,

    #[arg(long, value_parser=clap::value_parser!(u8).range(1..=2),)]
    pub part: u8,
}
