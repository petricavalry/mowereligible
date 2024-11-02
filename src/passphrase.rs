use crate::cli;
use rand::{seq::SliceRandom, Rng};
use std::fs::{self, File};
use std::io;
use std::io::prelude::*;
use std::path::Path;

const URL: &str = "https://www.eff.org/files/2016/07/18/eff_large_wordlist.txt";

async fn download_wordlist<T, P>(url: T, path: P, proxy: Option<String>) -> Result<String, reqwest::Error>
where
    T: reqwest::IntoUrl,
    P: AsRef<Path>,
{
    let mut builder = reqwest::Client::builder();
    if let Some(proxy) = proxy {
        builder = builder.proxy(reqwest::Proxy::all(proxy)?)
    };
    let client = builder.build().expect("failed to build client");
    let content = client.get(url).send().await?.text().await?;
    let mut file = File::create(&path).expect("failed to create directory");
    file.write_all(content.as_bytes())
        .expect("failed to write content");
    Ok(content)
}

async fn get_wordlist<T, P>(url: T, path: P, proxy: Option<String>) -> Result<Vec<String>, io::Error>
where
    T: reqwest::IntoUrl,
    P: AsRef<Path>,
{
    let content = if path.as_ref().exists() {
        fs::read_to_string(path)?
    } else {
        download_wordlist(url, path, proxy).await.unwrap()
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

pub async fn passphrase<P>(path: P, opts: cli::PassphraseArgs)
where
    P: AsRef<Path>,
{
    let wordlist = get_wordlist(URL, path, opts.proxy)
        .await
        .expect("failed to get wordlist from network or disk");

    let mut words: Vec<String> = vec![];
    let mut rng = rand::thread_rng();
    let number = if opts.number {
        rng.gen_range(0..opts.length)
    } else {
        opts.length
    };
    for index in 0..opts.length {
        let mut word = wordlist
            .choose(&mut rng)
            .expect("failed to select word from wordlist")
            .to_string();
        if opts.uppercase {
            word.get_mut(0..1).unwrap().make_ascii_uppercase();
        }
        if index == number {
            word.push_str(&rng.gen_range(0..9).to_string());
        }
        words.push(word);
    }
    println!("{}", words.join(&opts.separator));
}
