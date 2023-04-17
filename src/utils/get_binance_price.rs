use reqwest::Client;
use serde_json::Value;

const BINANCE_API: &str = "https://api.binance.com/api/v3/ticker/price";

pub async fn get_binance_price(client: &Client, symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {

    
    let url = format!("{}?symbol={}", BINANCE_API, symbol);
    let response = client.get(url).send().await?.text().await?;
    println!("Binance response: {}", response); // 追加
    let json: Value = serde_json::from_str(&response)?;
    let price: f64 = json["price"].as_str().unwrap().parse().unwrap();
    Ok(price)
}
