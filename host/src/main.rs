mod dummy;

use ethers_core::types::U512;
// load ELF and image ID
use methods::{VWAP_IMPL_ELF, VWAP_IMPL_ID};
use risc0_zkvm::{default_prover, ExecutorEnv};

use dummy::load_dummy;

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // load dummy
    let dummy_data = load_dummy();
    // env builder
    let mut env_builder = ExecutorEnv::builder();
    // write length
    env_builder.write(&dummy_data.len()).unwrap();
    // write date
    for d in dummy_data {
        env_builder.write(&d).unwrap();
    }
    // build env
    let env = env_builder.build().unwrap();

    // Obtain the default prover.
    let prover = default_prover();

    // Produce a receipt by proving the specified ELF binary.
    let receipt = prover
        .prove(env, VWAP_IMPL_ELF)
        .unwrap();

    // For example:
    let output: U512 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt
        .verify(VWAP_IMPL_ID)
        .unwrap();

    println!("the vwap of sqrtPriceX96 is {:?}", output);
}
