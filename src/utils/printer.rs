use anyhow::Result;
use colored::*;
use mime::Mime;
use reqwest::{header, Request, Response};

pub fn print_request(req: &Request) -> Result<()> {
    let method = req.method();
    let url = req.url().to_string().cyan();
    println!("{} {}\n", format!("{:?}", req.method()).blue(), url);

    for (name, value) in req.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();

    if method == "POST" || method == "PUT" {
        // Convert the body to a string
        if let Some(body) = req.body() {
            let body_str = std::str::from_utf8(body.as_bytes().unwrap()).unwrap();
            println!("{}", jsonxf::pretty_print(body_str).unwrap().purple());
        }
    }
    println!();

    Ok(())
}

fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}

fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    println!();
}

fn print_body(m: Option<Mime>, body: &String) {
    match m {
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().yellow())
        }
        _ => println!("{}", body),
    }
}

fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

pub async fn print_resp(resp: Response) -> Result<()> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}
