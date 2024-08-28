use rand::{
    distributions::{self, Alphanumeric},
    random, thread_rng, Rng,
};
use std::{
    char,
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::{sync::Semaphore, task::JoinHandle};

use crate::requester::Requester;

pub struct Fuzzer {
    pub target_ip: String,
    pub wordlist: Arc<Vec<String>>,
    pub threadpool: Arc<Semaphore>,
    pub handles: Vec<JoinHandle<()>>,
}

impl Fuzzer {
    pub async fn new(
        target_ip: String,
        wordlist: Arc<Vec<String>>,
        threadpool: Arc<Semaphore>,
    ) -> Self {
        let handles = Vec::new();

        return Self {
            target_ip,
            wordlist,
            threadpool,
            handles,
        };
    }

    pub async fn progress(reqs: Arc<AtomicUsize>, len: Arc<AtomicUsize>) {
        // let mut interval = interval(Duration::from_secs(1));
        loop {
            // interval.tick().await;
            let curr = reqs.load(Ordering::SeqCst);
            let total = len.load(Ordering::SeqCst);

            print!("\r[{}] => ", curr);
            std::io::stdout().flush().unwrap();

            if curr == total {
                println!();
                break;
            }
        }
    }

    pub async fn heuristic(target_ip: String, domain: String, agent: String) -> u64 {
        let noise = format!("{}.{}",
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(32)
                .map(char::from)
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .concat(),
            domain
        );


        let client = Requester::new(&noise, target_ip, agent).await;
        // println!("{}", noise);
        // println!("{:#?}", client);
        // let mut buffer = String::new();

        let response = Requester::client(client).await.unwrap().content_length().unwrap();
        println!("heuristic content_length result: {:#?}", response);
        return response;



        //
        // return 1;
    }
}
