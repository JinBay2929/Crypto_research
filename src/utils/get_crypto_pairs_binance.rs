use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Symbol {
    symbol: String,
}


pub async fn get_crypto_pairs_binance() -> Result<(), Error> {
    let url = "https://api.binance.com/api/v3/exchangeInfo";
    let response = reqwest::get(url).await?.json::<BinanceResponse>().await?;

    for symbol in response.symbols {
        println!("{}", symbol.symbol);
    }

    Ok(())
}

#[derive(Deserialize, Debug)]
struct BinanceResponse {
    symbols: Vec<Symbol>,
}