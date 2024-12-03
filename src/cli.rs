use std::fmt;

#[derive(clap::Parser)]
#[command(author, version, about)]
pub struct Cli {
    #[arg(long)]
    pub day: u8,

    #[arg(long)]
    pub part: Option<Part>,
}

#[derive(clap::ValueEnum, Clone)]
pub enum Part {
    #[clap(name = "1")]
    One,
    #[clap(name = "2")]
    Two,
}

impl std::convert::TryFrom<u8> for Part {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Part::One as u8 => Ok(Part::One),
            x if x == Part::Two as u8 => Ok(Part::Two),
            _ => Err(()),
        }
    }
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}
