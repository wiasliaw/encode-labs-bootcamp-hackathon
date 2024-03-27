use ethers_core::types::U256;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Dummy {
    pub volume: U256,
    pub sqrt_price: U256,
}

impl Dummy {
    pub fn from(v: U256, p: U256) -> Self {
        Self {
            volume: v,
            sqrt_price: p,
        }
    }
}
