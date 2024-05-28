use clap::Args;

use crate::utils::kv_pair;
use crate::utils::validator;

/// feed get with an url and we will retrieve the response for you
#[derive(Args, Debug)]
pub struct Get {
    /// url to make the GET request
    #[arg(value_parser = validator::parse_url)]
    pub url: String,
}

/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Args, Debug)]
pub struct Post {
    /// url to make the POST request
    #[arg(value_parser = validator::parse_url)]
    pub url: String,
    /// key=value pairs to post as JSON
    #[arg(value_parser = validator::parse_kv)]
    pub body: Vec<kv_pair::KvPair>,
}
