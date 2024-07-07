#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use reqwest::Client;
use std::fs::read_to_string;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct StockRaw {
    c: f64, // current price
    h: f64, // high price of the day
    l: f64, // low price of the day
    o: f64, // open price of the day
    pc: f64, // previous close price
}
async fn stock_request(token: &str, symbol: &str) -> Result<StockRaw, Box<dyn std::error::Error>> {
    let url = format!("https://finnhub.io/api/v1/quote?symbol={}&token={}", symbol, token);
    eprintln!("Fetching {url:?}...");

    let client = Client::new();
    let res = client.get(url)
        .send() // confirm request using send()
        .await.unwrap();

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    match res.status() {
        reqwest::StatusCode::OK => {
            // on success, parse our JSON to an APIResponse
            match res.json::<StockRaw>().await {
                Ok(parsed) => return Ok(parsed),
                Err(_) => panic!("Symbol, {:?} didn't match the shape we expected.", symbol),
            };
        }
        reqwest::StatusCode::UNAUTHORIZED => {
            panic!("Need to grab a new token");
        }
        other => {
            panic!("Uh oh! Something unexpected happened: {:?}", other);
        }
    }
}

#[allow(dead_code)]
pub struct Stock {
    val: StockRaw, // will carry all data about prices of a certain stock
    symbol: String, // holds the stock's symbol helps to differentiate
}
impl Stock {
    pub async fn new(symbol: String, token: String,) -> Result<Stock, Box<dyn std::error::Error>> {
        return Ok(Stock { val: stock_request(&token, &symbol).await?, symbol })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    let token = read_to_string("token.txt")?;
    let aaple = Stock::new("AAPL".to_string(), token).await?;
    println!("{}",aaple.val.c);
    Ok(())
}
