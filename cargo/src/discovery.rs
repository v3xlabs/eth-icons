use crate::{
    identity::{Address, NetworkId},
    IconClient,
    Icon,
    Error,
};

#[derive(Debug, Clone)]
pub enum IconQuery {
    // Ethereum Mainnet, Sepolia, etc.
    Network(NetworkId),
    // ETH, sepETH, etc.
    Native(NetworkId),
    // wETH, etc.
    ERC20(NetworkId, Address),
}

#[derive(Debug, Clone)]
pub enum DiscoveryResult {
    Found(Icon),
    NotFound,
    Unsupported,
}

impl From<Icon> for DiscoveryResult {
    fn from(icon: Icon) -> Self {
        DiscoveryResult::Found(icon)
    }
}

impl From<Option<Icon>> for DiscoveryResult {
    fn from(icon: Option<Icon>) -> Self {
        match icon {
            Some(icon) => DiscoveryResult::Found(icon),
            None => DiscoveryResult::NotFound,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiscoveryResults {
    pub query: IconQuery,
    pub results: Vec<DiscoveryResult>,
}

pub trait DiscoveryMechanism {
    fn url(&self, query: &IconQuery) -> Option<String>;

    fn fetch(
        &self,
        client: &IconClient,
        query: IconQuery,
    ) -> impl std::future::Future<Output = Result<DiscoveryResult, Error>> + Send;
}
