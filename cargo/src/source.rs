use crate::{
    Error, IconClient,
    identity::{Address, NetworkId},
    result::IconResult,
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

pub trait IconSource {
    fn url(&self, query: &IconQuery) -> Option<String>;

    fn fetch(
        &self,
        client: &IconClient,
        query: IconQuery,
    ) -> impl std::future::Future<Output = Result<IconResult, Error>> + Send;
}
