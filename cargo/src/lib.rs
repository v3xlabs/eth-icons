pub mod client;
pub mod discovery;
pub mod icon;
pub mod identity;
pub mod modules;
pub mod error;

pub use {
    error::Error,
    client::IconClient,
    icon::Icon,
    discovery::IconQuery
};
