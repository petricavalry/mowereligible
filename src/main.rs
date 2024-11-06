use clap::Parser;
use directories::ProjectDirs;
use rand::Rng;
use std::fs;

mod cli;
mod passphrase;

#[tokio::main]
async fn main() {
    let proj_dir = ProjectDirs::from("com.github", "petricavalry", "mowereligible")
        .expect("failed to get configuration path");
    let cache_dir = proj_dir.cache_dir();
    fs::create_dir_all(cache_dir).expect("failed to create configuration path");
    let path = cache_dir.join("eff_large_wordlist.txt");

    let args = cli::Cli::parse();

    if args.quiet {
        log::set_max_level(log::LevelFilter::Error);
    };

    match args.command {
        cli::Commands::Number(opts) => {
            let number = rand::thread_rng().gen_range(opts.start..opts.end);
            println!("{}", number);
        }
        cli::Commands::Passphrase(opts) => {
            passphrase::passphrase(path, opts).await;
        }
    };
}
