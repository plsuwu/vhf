use anyhow::Error;
use clap::Parser;
use reqwest::{Request, Response, StatusCode};
use std::{
    collections::HashMap,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use tokio::{
    sync::{Semaphore, SemaphorePermit},
    task,
};
use vhost_enumerator::{
    fuzzer::Fuzzer,
    parsers::{Agent, Url, Wordlist},
    requester::Requester,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    ip: String,

    #[arg(short, long, default_value_t=1.to_string())]
    wordlist: String,

    #[arg(short, long, default_value_t=0.to_string())]
    agent: String,

    #[arg(short, long, default_value_t = 25)]
    threads: usize,

    #[arg(short, long, default_value_t = false)]
    filter: bool,

    #[arg(long, default_value_t = false)]
    no_tls: bool,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    // parse wordlist via the index/path handler func
    let user_agent = Agent::from(&args.agent)?;
    let ip_url = Url::from(&args.ip, args.no_tls)?;
    // let base_domain = args.url;

    let wordlist: Arc<Vec<String>> = Wordlist::from(&args.wordlist).await?;
    let threadpool = Arc::new(Semaphore::new(args.threads));
    let results = Arc::new(Mutex::new(HashMap::new()));

    // Fuzzer object is not necessary atm but provides nice encapsulation for the overall fuzz run
    // let fuzzer = Fuzzer::new(target_ip, wordlist, threadpool);

    let mut req_handles = Vec::new();

    // let mut print_handles = Vec::new();

    let requests_total = Arc::new(AtomicUsize::new(wordlist.len()));
    let req_count = Arc::new(AtomicUsize::new(0));

    // let _progress = {
    //     let req_count = req_count.clone();
    //     tokio::spawn(async move {
    //         Fuzzer::progress(req_count, requests_total).await;
    //     });
    // };

    let filter_heuristic =
        Fuzzer::heuristic(ip_url.clone(), args.url.clone(), user_agent.clone()).await;

    for (i, word) in wordlist.iter().enumerate() {
        let word = word.clone();
        let filtered = filter_heuristic.clone();
        let base_domain = args.url.clone();
        let index = i.clone();
        let req_count = req_count.clone();
        let threadpool = Arc::clone(&threadpool);
        let agent = user_agent.clone();
        let url = ip_url.clone();
        let res = results.clone();

        let handle = task::spawn(async move {
            // run requests
            let permit = threadpool.acquire().await.unwrap();
            proc_dbg(
                word,
                base_domain,
                index,
                url,
                agent,
                req_count,
                permit,
                res,
                filtered,
            )
            .await;
        });

        req_handles.push(handle);
    }

    for handle in req_handles {
        handle.await?;
    }
    //
    // println!();
    //
    println!("{:#?}", results.lock().unwrap());

    return Ok(());
}

async fn proc_dbg(
    word: String,
    domain: String,
    index: usize,
    url: String,
    agent: String,
    counter: Arc<AtomicUsize>,
    permit: SemaphorePermit<'_>,
    result: Arc<Mutex<HashMap<String, StatusCode>>>,
    filtered: u64,
) {
    print!("\r\r\r");
    std::io::stdout().lock().flush().unwrap();

    // write!(lock, "").unwrap();
    // lock.flush().unwrap();

    counter.fetch_add(1, Ordering::SeqCst);
    let curr = counter.load(Ordering::SeqCst);
    // let dbug = "https://google.com".to_string();

    let subdomain = format!("{}.{}", word, domain);
    // let subdomain = format!("www.google.com");

    let client = Requester::new(&subdomain, url, agent).await;
    // println!("{:#?}", client);
    let mut buffer = String::new();

    let response = Requester::client(client).await;

    match response {
        Ok(r) => {
            if r.status() == 200 {
                if r.content_length() != Some(filtered) {
                    result.lock().unwrap().insert(subdomain.clone(), r.status());
                    buffer.push_str(&format!(
                        "[+]  got status '{}' on '{}' ({:?} bytes)",
                        r.status(),
                        &subdomain,
                        r.content_length()
                    ));
                } else {
                    buffer.push_str(&format!("filtered response [{}] for '{}'", r.status(), &subdomain));
                }
            }
        }
        Err(r) => {
            buffer.push_str(&format!("[!] {:#?} - {}\r", r, &subdomain));
        }
    }


    let mut lock = std::io::stdout().lock();
    write!(lock, "\r[{}] => {}\r", curr, buffer).unwrap();
    lock.flush().unwrap();
}
