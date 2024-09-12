use clap::Parser;
use directories::ProjectDirs;
use rand::{seq::SliceRandom, Rng};
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

mod cli;

const URL: &str = "https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt";

async fn download_wordlist<T, P>(url: T, path: P) -> Result<String, reqwest::Error>
where
    T: reqwest::IntoUrl,
    P: AsRef<Path>,
{
    let content = reqwest::get(url).await?.text().await?;
    let mut file = File::create(&path).expect("failed to create directory");
    file.write_all(content.as_bytes())
        .expect("failed to write content");
    Ok(content)
}

async fn get_wordlist<T, P>(url: T, path: P) -> Result<Vec<String>, io::Error>
where
    T: reqwest::IntoUrl,
    P: AsRef<Path>,
{
    let content = if path.as_ref().exists() {
        fs::read_to_string(path)?
    } else {
        download_wordlist(url, path).await.unwrap()
    };
    let mut wordlist = Vec::new();
    for line in content.lines() {
        wordlist.push(
            line.split_whitespace()
                .last()
                .expect("failed to get last word from line")
                .to_string(),
        );
    }
    Ok(wordlist)
}

#[tokio::main]
async fn main() {
    let proj_dir = ProjectDirs::from("com.github", "petricavalry", "mowereligible")
        .expect("failed to get configuration path");
    let config_dir = proj_dir.config_dir();
    fs::create_dir_all(config_dir).expect("failed to create configuration path");

    let path = config_dir.join("eff_large_wordlist.txt");

    let wordlist = get_wordlist(URL, path)
        .await
        .expect("failed to download EFF Large Wordlist");

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
