use std::collections::HashMap;

use anyhow::Result;
use once_cell::sync::Lazy;

use crate::command_args::*;

static API_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    reqwest::Client::builder()
        .build()
        .expect("Failed to create Client for httpie")
});

pub async fn get(args: &Get) -> Result<()> {
    let resp = API_CLIENT.get(&args.url).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}

pub async fn post(args: &Post) -> Result<()> {
    let mut body = HashMap::new();
    for kv in &args.body {
        body.insert(&kv.k, &kv.v);
    }
    let resp = API_CLIENT.post(&args.url).json(&body).send().await?;
    println!("{:?}", resp.text().await?);
    Ok(())
}
