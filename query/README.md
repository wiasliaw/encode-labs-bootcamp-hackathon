# Uniswap V3 Query

Query specified event and perform VWAP calculation.

## event

- Uniswap V3 Pool

```solidity
event Swap(
    address indexed sender,    // topic 1
    address indexed recipient, // topic 2
    int256 amount0,            // data[0..32]
    int256 amount1,            // data[32..64]
    uint160 sqrtPriceX96,      // data[64..96]
    uint128 liquidity,
    int24 tick
);
```

## dummy data

- block: 19524430
- two txs with two event emitted
  - [0x38aaf7c5a9043850def68c4a66bec4fda3bf8db3ae6512492a9f195203414eaf](https://etherscan.io/tx/0x38aaf7c5a9043850def68c4a66bec4fda3bf8db3ae6512492a9f195203414eaf)
  - [0xe63d867f4c83931a2ae82e421806c062aeeb91d9bfd0eee7ec5f6f26edc2a9c2](https://etherscan.io/tx/0xe63d867f4c83931a2ae82e421806c062aeeb91d9bfd0eee7ec5f6f26edc2a9c2)

## output

```txt
vwap_sqrt_price_x96: 1324968562874667106663579553125468

VWAP price of WETH/USDC
= 1 / ((1324968562874667106663579553125468 / 2^96)^2 / 10^12)
= 3575.592514...
```
