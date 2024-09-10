use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Number(NumberArgs),
    String(StringArgs),
}

#[derive(Args)]
pub struct NumberArgs {
    pub start: i32,

    pub end: i32,
}

#[derive(Args)]
pub struct StringArgs {
    #[arg(short, long, default_value = "6")]
    pub length: u8,

    #[arg(short, long, default_value = "-")]
    pub separator: String,
}
