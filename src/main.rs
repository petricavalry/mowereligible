use clap::Parser;
use mowereligibl::cli::{Cli, Commands};
use rand::{Rng, seq::SliceRandom};
use std::io::{self, BufRead};
use std::path::Path;

fn wordlist<T>(t: T) -> io::Result<Vec<String>>
where
    T: AsRef<Path>,
{
    let file = std::fs::File::open(t)?;
    let lines = io::BufReader::new(file).lines();
    let mut wordlist = Vec::new();
    for line in lines {
        wordlist.push(
            line?
                .split_whitespace()
                .last()
                .expect("cannot read line")
                .to_string(),
        );
    }
    Ok(wordlist)
}

fn main() {
    let args = Cli::parse();
    let wordlist = wordlist("eff_large_wordlist.txt").expect("");

    match args.command {
        Commands::Number(opts) => {
            let number = rand::thread_rng().gen_range(opts.start..opts.end);
            println!("{}", number);
        }
        Commands::String(opts) => {
            let mut words: Vec<String> = vec![];
            for _ in 0..opts.length {
                words.push(
                    wordlist
                        .choose(&mut rand::thread_rng())
                        .expect("cannot get wordlist")
                        .to_string(),
                );
            }
            println!("{}", words.join(&opts.separator));
        }
    }
}
