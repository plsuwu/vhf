use anyhow::{Context, Error};
use clap::Parser;
use futures::future;
use reqwest::{
    self,
    header::{HOST, USER_AGENT},
    Client,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio;
use tokio::sync::Semaphore;
use tokio::time::{interval, Duration};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    ip: String,

    #[arg(short, long)]
    wordlist: Option<String>,

    #[arg(short, long, default_value_t = 50)]
    concurrency: usize,
}

async fn print_progress(req_count: Arc<AtomicUsize>) {
    let mut interval = interval(Duration::from_secs(3));
    loop {
        interval.tick().await;
        let count = req_count.load(Ordering::SeqCst);
        println!("\rmade [ {} ] requests..", count);
        std::io::stdout().flush().unwrap();
    }
}

async fn requester(
    url: &str,
    ip: &str,
    subdomain: &str,
    client: Client,
    semaphore: Arc<Semaphore>,
    request_count: Arc<AtomicUsize>,
) -> Result<bool, reqwest::Error> {
    let _permit = semaphore
        .acquire()
        .await
        .expect("unable to acquire thread permit.");
    request_count.fetch_add(1, Ordering::SeqCst);

    let vhost: String = format!("{}.{}", subdomain, url);
    let ip_as_url: String = format!("http://{}", ip);

    let mut vhost_status = false;

    let res = client
        .get(&ip_as_url)
        .header(HOST, &vhost)
        .header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36")
        .send()
        .await?;

    if res.status().is_success() {
        println!("[{}]: {}", res.status(), vhost);
        vhost_status = true;
    }

    return Ok(vhost_status);
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let arguments = Args::parse();
    let ip_arc = Arc::new(arguments.ip);

    let wordlist_filepath = arguments
        .wordlist
        .expect("err reading filepath for wordlist");
    let wordlist_file = File::open(&wordlist_filepath)
        .with_context(|| format!("unable to open {}", wordlist_filepath))?;
    let reader = BufReader::new(wordlist_file);

    let request_count = Arc::new(AtomicUsize::new(0));

    let progress = {
        let request_count_clone = request_count.clone();
        tokio::spawn(async move {
            print_progress(request_count_clone).await;
        })
    };

    let client = Client::new();
    let semaphore = Arc::new(Semaphore::new(arguments.concurrency));
    let mut tasks = vec![];

    for line in reader.lines() {
        let subdomain = line?;
        if subdomain.is_empty() {
            continue;
        }

        let client_clone = client.clone();
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
            )
            .await;
        });

        tasks.push(task);
    }

    let _: Vec<_> = future::join_all(tasks).await.into_iter().collect();
    progress.abort();

    Ok(())
}
