use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    #[arg(short, long)]
    pub quiet: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Number(NumberArgs),
    Passphrase(PassphraseArgs),
}

#[derive(Args)]
pub struct NumberArgs {
    pub start: i32,

    pub end: i32,
}

#[derive(Args)]
pub struct PassphraseArgs {
    #[arg(short, long, default_value = "6")]
    pub length: u8,

    #[arg(short, long, default_value = "-")]
    pub separator: String,

    #[arg(short, long)]
    pub number: bool,

    #[arg(short, long)]
    pub uppercase: bool,

    #[arg(short = 'x', long)]
    pub proxy: Option<String>, // TODO: using specific type
}
