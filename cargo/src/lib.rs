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
