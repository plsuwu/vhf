use std::{
    io::Write,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};
use tokio::time::{interval, Duration};

pub async fn print_progress(req_count: Arc<AtomicUsize>) {
    let mut interval = interval(Duration::from_secs(1));
    loop {
        interval.tick().await;
        let count = req_count.load(Ordering::SeqCst);
        println!("\rmade [ {} ] requests..", count);
        std::io::stdout().flush().unwrap();
    }
}
