use clap::Parser;
use directories::ProjectDirs;
use rand::{seq::SliceRandom, Rng};
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead};
use std::path::Path;

mod cli;

async fn get_wordlist<T, P>(url: T, path: P) -> Result<Vec<String>, io::Error>
where
    T: reqwest::IntoUrl,
    P: AsRef<Path>,
{
    if !path.as_ref().exists() {
        let resp = reqwest::get(url)
            .await
            .expect("fail to download wordlist")
            .text()
            .await
            .expect("fail to get response");
        let mut file = File::create(&path).expect("fail to create wordlist file");
        file.write_all(resp.as_bytes()).expect("fail to write file");
    }

    let file = std::fs::File::open(path)?;
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

#[tokio::main]
async fn main() {
    let proj_dir = ProjectDirs::from("com.github", "petricavalry", "mowereligible")
        .expect("fail to get configuration path");
    let config_dir = proj_dir.config_dir();
    fs::create_dir_all(config_dir).expect("fail to create configuration path");

    let path = config_dir.join("eff_large_wordlist.txt");
    let url = "https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt";
    let wordlist = get_wordlist(url, path).await.expect("unknown");

    let args = cli::Cli::parse();

    match args.command {
        cli::Commands::Number(opts) => {
            let number = rand::thread_rng().gen_range(opts.start..opts.end);
            println!("{}", number);
        }
        cli::Commands::String(opts) => {
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
    };
}
