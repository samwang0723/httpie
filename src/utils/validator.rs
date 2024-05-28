use anyhow::Result;
use reqwest::Url;

use crate::utils::kv_pair::KvPair;

pub fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}

pub fn parse_kv(s: &str) -> Result<KvPair> {
    s.parse()
}
