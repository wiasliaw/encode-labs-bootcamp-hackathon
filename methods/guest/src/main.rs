#![no_main]

use ethers_core::types::{U256, U512};
use risc0_zkvm::guest::env;
use dummy::Dummy;

risc0_zkvm::guest::entry!(main);

fn main() {
    // read the input
    let len: usize = env::read();
    let mut sum_pv = U512::zero();
    let mut total_volume = U256::zero();

    for _i in 0..len {
        let value: Dummy = env::read();
        sum_pv = sum_pv.checked_add(value.sqrt_price.full_mul(value.volume)).unwrap();
        total_volume = total_volume.checked_add(value.volume).unwrap();
    }

    // calculate vwap
    let vwap = sum_pv.checked_div(U512::from(total_volume)).expect("overflow");

    // write public output to the journal
    env::commit(&vwap);
}
