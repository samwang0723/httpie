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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_kv_invalid() {
        let kv_str = "invalid_kv";
        let result = parse_kv(kv_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_url_valid() {
        let url = "https://www.example.com";
        let result = parse_url(url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), url);
    }
}
