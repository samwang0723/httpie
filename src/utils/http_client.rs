use std::collections::HashMap;

use anyhow::Result;
use once_cell::sync::Lazy;

use crate::utils::command_args::*;
use crate::utils::printer::*;

static API_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-POWERED-BY", "Rust".parse().unwrap());
    headers.insert(reqwest::header::USER_AGENT, "Rust Httpie".parse().unwrap());
    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to create Client for httpie")
});

pub async fn get(args: &Get) -> Result<()> {
    let resp = API_CLIENT.get(&args.url).send().await?;
    print_resp(resp).await
}

pub async fn post(args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for kv in &args.body {
        body.insert(&kv.k, &kv.v);
    }
    let resp = API_CLIENT.post(&args.url).json(&body).send().await?;
    print_resp(resp).await
}
