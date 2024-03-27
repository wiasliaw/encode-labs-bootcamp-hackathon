use ethers_core::types::U256;
use dummy::Dummy;

pub fn load_dummy() -> Vec<Dummy> {
    let a1 = U256::from_dec_str("2788578251457772992").unwrap();
    let p1 = U256::from_dec_str("1324968336584293285703398427761193").unwrap();
    let a2 = U256::from_dec_str("297375000000000000").unwrap();
    let p2 = U256::from_dec_str("1324970684870177661952189920610756").unwrap();

    vec![
        Dummy::from(a1, p1),
        Dummy::from(a2, p2),
    ]
}
