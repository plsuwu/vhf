use vhfuzz::progress;
use vhfuzz::requester::requester;
use vhfuzz::default_words::DEFAULT_WORDLIST;

use anyhow::{Context, Error};
use clap::Parser;
use futures::future;
use reqwest::{self, Client};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    sync::{atomic::AtomicUsize, Arc},
};
use tokio;
use tokio::sync::Semaphore;

pub const IMPERSONATE: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    ip: String,

    #[arg(short, long)]
    wordlist: Option<String>,

    #[arg(short, long, default_value=IMPERSONATE)]
    agent: String,

    #[arg(short, long, default_value_t = 25)]
    threads: usize,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arguments = Args::parse();
    let ip_arc = Arc::new(arguments.ip);
    let agent = arguments.agent;
    let mut wordlist_filepath: String;

    if arguments.wordlist.is_some() {
        wordlist_filepath = DEFAULT_WORDLIST.to_string();
    } else {
        wordlist_filepath = arguments.wordlist.expect("can't set wordlist path to provided wordlist! (this shouldn't happen).");
    }
    let wordlist_file = File::open(&wordlist_filepath)
        .with_context(|| format!("unable to open {}", wordlist_filepath))?;
    let reader = BufReader::new(wordlist_file);

    let request_count = Arc::new(AtomicUsize::new(0));

    let progress = {
        let request_count_clone = request_count.clone();
        tokio::spawn(async move {
            progress::print_progress(request_count_clone).await;
        })
    };

    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(arguments.threads));
    let mut tasks = vec![];

    for line in reader.lines() {
        let subdomain = line?;
        if subdomain.is_empty() {
            continue;
        }

        let client_clone = client.clone();
        let agent_clone = agent.clone();
        let semaphore_clone = semaphore.clone();
        let url_clone = arguments.url.clone();
        let ip_clone = Arc::clone(&ip_arc);
        let request_count_clone = request_count.clone();

        let task = tokio::spawn(async move {
            let _ = requester(
                &url_clone,
                ip_clone.as_str(),
                &subdomain,
                client_clone,
                semaphore_clone,
                request_count_clone,
                &agent_clone,
            )
            .await;
        });

        tasks.push(task);
    }

    let _: Vec<_> = future::join_all(tasks).await.into_iter().collect();
    progress.abort();

    return Ok(());
}
