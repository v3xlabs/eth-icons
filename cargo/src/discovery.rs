use crate::identity::{NetworkId, Address};

#[derive(Debug, Clone)]
pub enum DiscoveryQuery {
    // Ethereum Mainnet, Sepolia, etc.
    Network(NetworkId),
    // ETH, sepETH, etc.
    Native(NetworkId),
    // wETH, etc.
    ERC20(NetworkId, Address),
}

#[derive(Debug, Clone)]
pub struct DiscoveryResults {
    pub query: DiscoveryQuery,
    pub results: Vec<DiscoveryResult>
}

#[derive(Debug, Clone)]
pub struct DiscoveryResult {
    pub icon: Option<String>,
    pub metadata: Option<String>
}

impl DiscoveryResult {
    pub fn is_empty(&self) -> bool {
        self.icon.is_none() && self.metadata.is_none()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    #[error("Not found")]
    NotFound,
    #[error("HTTP error: {0}")]
    HttpError(reqwest::Error),
    #[error("Internal error: {0}")]
    InternalError(String),
}

pub trait DiscoveryMechanism {
    fn discover(&self, query: DiscoveryQuery) -> impl std::future::Future<Output = Result<Option<DiscoveryResult>, DiscoveryError>> + Send;
}
