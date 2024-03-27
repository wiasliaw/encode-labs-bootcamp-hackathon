use ethers::{
    core::types::{Address, Filter, U256},
    providers::{Http, Middleware, Provider}, types::{I256, U512},
};
use eyre::{Ok, Result};
use std::sync::Arc;

const HTTP_URL: &str = "https://rpc.flashbots.net";
// uniswap v3 pool of WETH/USDC
const POOL_ADDRESS: &str = "0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640";
// const USDC_ADDRESS: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"; // token 0
// const WETH_ADDRESS: &str = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"; // token 1

#[tokio::main]
async fn main() -> Result<()> {
    let provider = Provider::<Http>::try_from(HTTP_URL)?;
    let client = Arc::new(provider);

    // variables
    let mut total_volume: U256 = U256::zero();
    let mut pv = Vec::new();

    // filter swap
    let swap_filter = Filter::new()
        .address(POOL_ADDRESS.parse::<Address>()?)
        .event("Swap(address,address,int256,int256,uint160,uint128,int24)")
        .from_block(19524430)
        .to_block(19524430);

    let swap_logs = client.get_logs(&swap_filter).await?;
    for log in swap_logs.iter() {
        let amount0 = I256::from_raw(U256::from_big_endian(&log.data[0..32]));
        let amount1 = I256::from_raw(U256::from_big_endian(&log.data[32..64]));
        let sqrt_price_x96 = U256::from_big_endian(&log.data[64..96]);
        println!("swap {:?}, {:?}, {:?}", amount0, amount1, sqrt_price_x96);

        let (_sign, amount1_abs) = amount1.into_sign_and_abs();
        pv.push(amount1_abs.full_mul(sqrt_price_x96));
        total_volume = total_volume.checked_add(amount1_abs).expect("overflow");
    }

    // calculate VWAP
    let sum_pv = pv.iter().fold(U512::zero(), |acc, x| acc.checked_add(*x).unwrap());
    let vwap = sum_pv.checked_div(U512::from(total_volume)).expect("overflow");
    println!("VWAP sqrt_price_x96: {}", vwap);
    Ok(())
}
