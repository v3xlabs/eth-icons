<h1 align="center">
    eth-icons
</h1>

<p align="center">
  A smol rust crate for fetching Ethereum asset icons from various sources.
</p>
<p align="center">
    <a href="https://docs.rs/eth-icons"><img src="https://img.shields.io/badge/Docs.rs-blue?logo=rust&color=brown&style=flat" alt="Documentation"></a>
    <a href="https://crates.io/crates/eth-icons"><img src="https://img.shields.io/badge/Crate.io-yellow?logo=data:image/x-icon;base64%2CiVBORw0KGgoAAAANSUhEUgAAABAAAAAQCAMAAAAoLQ9TAAACylBMVEUAAADqvWfotVLot1n/wi3lt1/grEfls1HnuFzXnCnXnS3luWTfzavgq0PlwXvltFXlrkTnt4KJYzl4XDz/yHwUEAsIBgQXEgyhek+DZ0j///9JOShFNibsxHnls1Llsk7ntVTpu2Lnv3Lntlf///bhrkrkslHnumTouFvpuV3otlbntFDotlbZoDDpuV7ot1fntlbnumLWnjLirEHlsUvcpz/lr0XlqznmsEjsvGDaozbmrDrgq0LpyIbmtlnlslDntVbdqUXmrTzfq0TbqUXcp0DirELgrUjLlS3OlCPiqTjkrULDjCLDiRfbozfrs0WUahe4fw3hqTrhrEVYRCiQaCfDlk2yjV0yJxppUDZ1WzqEYCKMZzqZd1GxjmgAAAALCAYEAwIwJRt2WjmKZi6fbxeifVKKZ0J3VjNlTC8AAAAAAAALCAVCMiI0KB1DMx+NajaIZDp5WDSFZkOEak4AAAAcFQ5xVzyffViAYDxkTjXnu2XnuV/dpzzjtFnlsk/ntFPntlnnu2Tou2Tbozfjrkfntlfou2XoumLntFLntlbnumLnu2Pir0znvWznumPntljotFDnskrkqzzmrkDbojTgrEfgqkHmr0XiqTvgpzbmrT3mrDjnrDjQliLTmCXVmyvhqDjJljHMmDLnrj/prjzorz/kt17nvGnZp0XGjBjKjxvQlSTepTfEkjHTnTXpsEHpsUTXojzbpj7SnTTDjSTGkSrIjhzjqjnepzrjrD7lr0biqz/cpTnHjBjKjx3MlCfNlSjJlC3GjiLhqTzls1Djrkbkqz3UnjTCkC++gxC/hRLFjB65gxm9hhvPliffpzjlrT3lrDrPmzTKljKwgy+4gBO/hhWrdxGlcxLFjR7kqTfnrDnorz7jqjvRnj2ndx6xeg6odAy/hxfkqjjqsUHmrkHUoUSxhUG7gxXgpzjRo1CmgEewh00nmQYaAAAAe3RSTlMAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMpetXWfCgBT9/9/d2TQg1v88FlCG7ZG2z+1xsOi9cbASl3xvbXGxHA1xwV0NgdFc/YHRrR2R004+ozJaHF1PauIQoaMmrr9vf0sk8MAQgcWnic+fGtSwsQPrm/UQuNPWZaAAABG0lEQVQY0wEQAe/+AAAAAAEdHh8gISIjJAIDBAUAAAAABiUmJ3t8KCkqKywHCAAAAAAJLX1+f4CBgoMuLzAxAAAAAAoyhIWGh4iJiouMMzQAAAsMDTWNjo+QkZI2k5Q3OAAODxA5OpWWl5iZmpucnTs8AD0+P0BBnp+goaKjpKWmQkMAREWnqKmqq6ytrq+wsYpGRwBISbKztLW2t7i5uru8vUpLAExNvr/AwcLDxMXGx8jJTk8AUFHKy8zNzs/Q0dKb09RSUwBUVdXW19jZ2tud3N3e31ZXAFhZWlvg4eLj5OXm5+hcXV4AX2BhYmNkZenq6+xmZ2hpEQAAamtsbW5vcO1xcnN0EhMUABUWABdqdXZ3eHl6GBkaGxxTKXBYeDUm8QAAAABJRU5ErkJggg==" alt="Crates.io"></a>
    <a href="https://github.com/v3xlabs/eth-icons"><img src="https://img.shields.io/badge/Repository-v3xlabs/eth--icons-blue?style=flat" alt="Repository"></a>
    <a href="#"><img src="https://img.shields.io/badge/Status-In%20Development-blue?style=flat" alt="Status: In Development"></a>
    <a href="#"><img src="https://img.shields.io/badge/License-LGPL--3.0-hotpink?style=flat" alt="License: LGPL-3.0"></a>
</p>

> [!IMPORTANT]
> eth-icons uses third party services to fetch iconography, not ethereum rpc calls.

Network (Mainnet, Sepolia, etc.), Native Asset (ETH, sepETH, etc.), and ERC20 (wETH, etc.) icons are an inherit non-standardized extension of the ethereum protocol.
To allow for easily integrating these into your application eth-icons aims to bring a set of helpers to obtain iconography from various sources.

## Quickstart

```sh
cargo add eth-icons
```

```rust
use eth_icons::{IconClient, IconQuery};

// Create a client
let client = reqwest::client();

// Create an icon client
let icons = IconClient::builder()
  .with_reqwest(client)
  .with_defaults();

let weth = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2";
let weth_icons = icons.resolve(IconQuery::ERC20(1.into(), weth.into())).await;
```

## Data Sources

Currently supported data sources include:

- [Avara](./src/modules/avara.rs) - supports ERC20s
- [Blockscout](./src/modules/blockscout.rs) - supports ERC20s
- [SafeWallet](./src/modules/safewallet.rs) - supports network and native token icons
- [Smoldapp](./src/modules/smoldapp.rs) - supports network, native tokens, and ERC20s
- [Zerion](./src/modules/zerion.rs) - supports ERC20s


## Selecting Sources

Every source is opt-in by default.

```rust
use eth_icons::{Address, Blockscout, IconClient, IconQuery, Zerion};

#[tokio::main]
pub async fn main() {
    let icons = IconClient::builder()
        .with_reqwest(reqwest::Client::new())
        .with_source(Blockscout)
        .with_source(Zerion)
        .build();
    let weth = Address("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string());

    let results = icons.resolve(IconQuery::ERC20(1, weth)).await;

    for result in &results {
        println!("{}: {:?}", result.source, result.result);
    }
}
```

## Documentation

You can read the documentation at [docs.rs/eth-icons](https://docs.rs/eth-icons).
