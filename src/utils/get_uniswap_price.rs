use web3::futures::Future;
use web3::types::{Address, U256};
use web3::Web3;

pub fn get_uniswap_price(web3: &Web3<web3::transports::Http>, token_a: Address, token_b: Address) -> Result<f64, web3::Error> {
    let contract_address: Address = "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f".parse().unwrap();
    let abi = include_bytes!("path/to/your/UniswapV2Pair_ABI.json");
    let contract = web3::contract::Contract::from_json(web3.eth(), contract_address, abi).unwrap();

    let result: (U256, U256, U256) = contract.query("getReserves", (), None, web3::contract::Options::default(), None).wait().unwrap();

    let reserve_a = result.0.low_u64() as f64;
    let reserve_b = result.1.low_u64() as f64;

    let price = reserve_a / reserve_b;

    Ok(price)
}
