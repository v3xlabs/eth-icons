/*!
 * `eth-icons` is an icon fetching library for EVM icons.
 *
 * Network (Mainnet, Sepolia, etc.), Native Asset (ETH, sepETH, etc.), and ERC20 (wETH, etc.) icons are an inherit non-standardized extension of the ethereum protocol.
 *
 * To allow for easily integrating these into your application eth-icons aims to bring a set of helpers to obtain iconography from various sources.
 *
 * ```rust
 * use eth_icons::{IconClient, IconQuery};
 *
 * #[tokio::main]
 * pub async fn main() {
 *   // Create a client
 *   let client = reqwest::Client::new();
 *
 *   // Create an icon client
 *   let icons = IconClient::builder()
 *     .with_reqwest(client)
 *     .with_defaults()
 *     .build();
 *
 *   let weth = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string();
 *   let weth_icons = icons.resolve(IconQuery::ERC20(1u64, weth)).await;
 * }
 * ```
 *
 * ## Selecting Sources
 *
 * Every source is opt-in by default.
 *
 * ```rust
 * use eth_icons::{Address, IconClient, IconQuery, modules::{Blockscout, Zerion}};
 *
 * #[tokio::main]
 * pub async fn main() {
 *     let icons = IconClient::builder()
 *         .with_reqwest(reqwest::Client::new())
 *         .with_source(Blockscout)
 *         .with_source(Zerion)
 *         .build();
 *
 *     let weth = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string();
 *     let results = icons.resolve(IconQuery::ERC20(1, weth)).await;
 *
 *     for source in &results {
 *         println!("{}: {:?}", source.source, source.result);
 *     }
 * }
 * ```
 *
 * ## Available Sources
 *
 * - [Avara](crate::modules::Avara) - supports ERC20s
 * - [Blockscout](crate::modules::Blockscout) - supports ERC20s
 * - [SafeWallet](crate::modules::SafeWallet) - supports network and native token icons
 * - [Smoldapp](crate::modules::Smoldapp) - supports network, native tokens, and ERC20s
 * - [Zerion](crate::modules::Zerion) - supports ERC20s
 *
 *
 */

pub mod client;
pub mod error;
pub mod fetcher;
pub mod icon;
pub mod identity;
pub mod modules;
pub mod result;
pub mod source;

pub use {
    client::{IconClient, builder::IconClientBuilder},
    error::Error,
    fetcher::IconFetcher,
    icon::Icon,
    identity::{Address, NetworkId},
    result::{IconResult, SourceResult},
    source::{IconQuery, IconSource},
};
