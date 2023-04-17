// mod utils;

// use utils::get_crypto_pairs_binance::get_crypto_pairs_binance;
// use utils::get_binance_price::get_binance_price;
// use reqwest::Client;

// #[tokio::main]
// async fn main() {
//     let client = Client::new();

//     let binance_btc_usd = get_binance_price(&client, "ETHBTC").await.expect("Failed to get Binance price");
//     println!("Binance BTC/USD: {}", binance_btc_usd);

//     get_crypto_pairs_binance().await.unwrap();
// }


mod utils;

use  utils::get_uniswap_price::get_uniswap_price;
use web3::transports::Http;
use web3::Web3;

fn main() {
    let transport = Http::new("https://mainnet.infura.io/v3/70cc2399d5c042c2a838475ee3789210").unwrap();
    let web3 = Web3::new(transport);

    let token_a = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".parse().unwrap();
    let token_b = "0x6B175474E89094C44Da98b954EedeAC495271d0F".parse().unwrap();

    let price = get_uniswap_price(&web3, token_a, token_b).unwrap();

    println!("Uniswap Price: {}", price);
}
