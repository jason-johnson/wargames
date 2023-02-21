use clap::{Parser, ValueEnum};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Scenario {
    PleskauQS,
    Pleskau,
    Watland,
    ReturnOfThePrince,
    ReturnOfThePrinceNicolle,
    Peipus,
    CrusadeOnNovgorod,
}

#[derive(Parser, Debug)]
#[command(term_width = 0)]
pub struct Args {
    /// Implicitly using `std::str::FromStr`
    #[arg(short = 'p', long)]
    player: Vec<String>,

    #[arg(long, value_enum)]
    scenario: Scenario,
}