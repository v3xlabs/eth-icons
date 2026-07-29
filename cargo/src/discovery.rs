use crate::identity::{NetworkId, Address};

pub enum DiscoveryQuery {
    // Ethereum Mainnet, Sepolia, etc.
    Network(NetworkId),
    // ETH, sepETH, etc.
    Native(NetworkId),
    // wETH, etc.
    ERC20(NetworkId, Address),
}

pub struct DiscoveryResults {
    query: DiscoveryQuery,
    results: Vec<DiscoveryResult>
}

pub struct DiscoveryResult {
    icon: Option<String>,
    metadata: Option<String>
}

impl DiscoveryResult {
    pub fn is_empty(&self) -> bool {
        self.icon.is_none() && self.metadata.is_none()
    }
}

pub trait DiscoveryMechanism {
    async fn discover(&self, query: DiscoveryQuery) -> DiscoveryResults;
}
