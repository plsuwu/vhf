use reqwest::{
    self,
    header::{HOST, USER_AGENT},
    Client,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tokio::sync::Semaphore;

pub async fn requester(
    url: &str,
    ip: &str,
    subdomain: &str,
    client: Client,
    semaphore: Arc<Semaphore>,
    request_count: Arc<AtomicUsize>,
    agent: &str,
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
        .header(USER_AGENT, agent)
        .send()
        .await?;

    if res.status().is_success() {
        println!("[{}]: {}", res.status(), vhost);
        vhost_status = true;
    }

    return Ok(vhost_status);
}
