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
pub struct DiscoveryResults {
    pub query: IconQuery,
    pub results: Vec<DiscoveryResult>,
}

#[derive(Debug, Clone)]
pub struct DiscoveryResult {
    pub icon: Option<Icon>,
    pub metadata: Option<String>,
}

impl DiscoveryResult {
    pub fn is_empty(&self) -> bool {
        self.icon.is_none() && self.metadata.is_none()
    }
}

pub trait DiscoveryMechanism {
    fn url(&self, query: &IconQuery) -> Option<String>;

    fn fetch(
        &self,
        client: &IconClient,
        query: IconQuery,
    ) -> impl std::future::Future<Output = Result<Option<DiscoveryResult>, Error>> + Send;
}
