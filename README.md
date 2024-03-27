# zk oracle with risc-zero

during encode lab bootcamp hackathon.

## VWAP

Gerenal formula

```txt
vwap = (V1 x P1 + V2 x P2… + Vn x Pn) / Total Volume
```

Use uniswap v3's **sqrtPriceX96** as price

```txt
vwap = (V1 x P1 + V2 x P2… + Vn x Pn) / Total Volume
```

## prelude

install rics-zero

```bash
cargo install cargo-binstall
cargo binstall cargo-risczero
cargo risczero install
```

## execute

```bash
cd ./host
cargo run

# log
# the vwap of sqrtPriceX96 is 1324968562874667106663579553125468
```

## reference

- definition of volume: https://www.investopedia.com/terms/v/volumeoftrade.asp
- https://chain.link/education-hub/twap-vs-vwap
- risc-zero: https://www.risczero.com/
