use anyhow::Error;
use regex::Regex;
use reqwest::{header::*, Client, Response};
use std::sync::Arc;
use tokio::sync::{Semaphore, SemaphorePermit};

// An example given the IP `10.10.11.28`, which wants to resolve to the domain 'sea.htb',
// might also run a different service on a subdomain at 'example.sea.htb'. By sending a
// GET request to http://10.10.11.28/ with the Host header set to 'example.sea.htb':
//
// GET / HTTP/1.1
// Host: example.sea.htb
// Accept-Language: en-US
// User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) ... ;
// Accept: */* ... ;
// Accept-Encoding: gzip, deflate, br
// Connection: close
//
//
// also, optionally filter out invalid sub-domains that always return the same response
// via the size of a response (maybe with the content-length header?)
// ... so if we query 'Host: <random, definitely not real subdomain>.sea.htb',
// we can filter out all results that return the same response, so we only see
// interesting results.

// is there actually a point to this struct and its function
// implementations or is this actually retarded i wonder
#[derive(Debug)]
pub struct RequestHeaders {
    pub vhost: HeaderValue,
    pub user_agent: HeaderValue,
    pub accept_lang: HeaderValue,
    pub accept_enc: HeaderValue,
    pub accept: HeaderValue,
    pub conn_state: HeaderValue,
}

impl RequestHeaders {
    pub fn raw(vhost: String, user_agent: String) -> Self {
        return Self {
            vhost: vhost.parse().unwrap(),
            user_agent: user_agent.parse().unwrap(),

            // make these into their own object that we can clone, maybe as an Arc?
            accept_lang: "en-US".parse().unwrap(),
            accept_enc: "gzip, deflate, br".parse().unwrap(),
            accept: "*/*".parse().unwrap(),
            conn_state: "close".parse().unwrap(),
        };
    }

    pub fn new(vhost: String, user_agent: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        let values = Self::raw(vhost, user_agent);

        headers.insert(HOST, values.vhost);
        headers.insert(USER_AGENT, values.user_agent);
        headers.insert(ACCEPT_LANGUAGE, values.accept_lang);
        headers.insert(ACCEPT_ENCODING, values.accept_enc);
        headers.insert(ACCEPT, values.accept);
        headers.insert(CONNECTION, values.conn_state);

        return headers;
    }
}

#[derive(Debug)]
pub struct Requester {
    // vhost: String, // includes base domain
    pub url: String,
    pub headers: HeaderMap,
}

impl Requester {
    pub async fn new(vhost: &str, url: String, agent: String) -> Self {
        let headers = RequestHeaders::new(vhost.to_string(), agent);

        return Self { url, headers };
    }

    pub async fn client(req: Requester) -> Result<Response, reqwest::Error> {
        let client = Client::new();
        let mut req_status = false;

        let res = client.get(req.url).headers(req.headers).send().await?;

        if res.status().is_success() {
            req_status = true;
        }

        return Ok(res);
    }
}
