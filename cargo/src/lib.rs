/*!
 * `eth-icons` is an icon fetching library for EVM icons.
 *
 * ```rust
 * use eth_icons::{};
 *
 * #[tokio::main]
 * pub async fn main() {
 *    let client = reqwest::client();
 * }
 * ```
 *
 * ## BYI Reqwest
 *
 * You can also bring your own `reqwest::client`.
 *
 * ```rust
 * use eth_icons::{};
 *
 * #[tokio::main]
 * pub async fn main() {
 *    let client = reqwest::client();
 * }
 * ```
 */

pub mod client;
pub mod error;
pub mod icon;
pub mod identity;
pub mod modules;
pub mod result;
pub mod source;

pub use {
    client::IconClient,
    error::Error,
    icon::Icon,
    result::IconResult,
    source::{IconQuery, IconSource},
};
