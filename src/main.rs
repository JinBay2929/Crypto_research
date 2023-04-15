mod utils;

use utils::get_crypto_pairs_binance::get_crypto_pairs_binance;
use reqwest::Client;
use serde_json::Value;

const BINANCE_API: &str = "https://api.binance.com/api/v3/ticker/price";
// const COINBASE_PRO_API: &str = "https://api.pro.coinbase.com/products/";

async fn get_binance_price(client: &Client, symbol: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let url = format!("{}?symbol={}", BINANCE_API, symbol);
    let response = client.get(url).send().await?.text().await?;
    println!("Binance response: {}", response); // 追加
    let json: Value = serde_json::from_str(&response)?;
    let price: f64 = json["price"].as_str().unwrap().parse().unwrap();
    Ok(price)
}

// async fn get_coinbase_pro_price(client: &Client, product_id: &str) -> Result<f64, Box<dyn std::error::Error>> {
//     let url = format!("{}/{}/ticker", COINBASE_PRO_API, product_id);
//     let response = client.get(url).send().await?.text().await?;
//     let json: Value = serde_json::from_str(&response)?;
//     let price: f64 = json["price"].as_str().unwrap().parse().unwrap();
//     Ok(price)
// }

#[tokio::main]
async fn main() {
    let client = Client::new();

    let binance_btc_usd = get_binance_price(&client, "ETHBTC").await.expect("Failed to get Binance price");
    println!("Binance BTC/USD: {}", binance_btc_usd);

    // let coinbase_btc_usd = get_coinbase_pro_price(&client, "ETH-BTC").await.expect("Failed to get Coinbase Pro price");
    // println!("Coinbase Pro BTC/USD: {}", coinbase_btc_usd);

    get_crypto_pairs_binance().await.unwrap();
    get_crypto_pairs_binance().await.unwrap();
}
