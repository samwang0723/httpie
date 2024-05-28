use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::collections::HashMap;
use std::str::FromStr;

use crate::utils::command_args::*;
use crate::utils::printer::*;

static API_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let mut headers = HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse().unwrap());
    headers.insert(reqwest::header::USER_AGENT, "Rust Httpie".parse().unwrap());
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create Client for httpie")
});

pub async fn get(args: &Get, headers: Vec<String>) -> Result<()> {
    let request = API_CLIENT
        .get(&args.url)
        .headers(compute_headers(headers))
        .build()?;
    let _ = print_request(&request);

    let resp = API_CLIENT.execute(request).await?;
    print_resp(resp).await
}

pub async fn post(args: &Post, headers: Vec<String>) -> Result<()> {
    let mut body = HashMap::new();
    for kv in &args.body {
        body.insert(&kv.k, &kv.v);
    }

    let request = API_CLIENT
        .post(&args.url)
        .headers(compute_headers(headers))
        .json(&body)
        .build()?;
    let _ = print_request(&request);

    let resp = API_CLIENT.execute(request).await?;
    print_resp(resp).await
}

fn compute_headers(inputs: Vec<String>) -> HeaderMap {
    let mut map = HeaderMap::new();
    for header in inputs {
        let kv: Vec<&str> = header.split(':').collect();
        if kv.len() == 2 {
            let key = kv[0].trim().to_string();
            let value = kv[1].trim().to_string();
            map.insert(
                HeaderName::from_str(&key).unwrap(),
                HeaderValue::from_str(&value).unwrap(),
            );
        }
    }

    map
}
