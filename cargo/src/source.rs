use crate::{
    Error, IconFetcher,
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

#[async_trait::async_trait]
pub trait IconSource: Send + Sync {
    fn name(&self) -> &'static str;

    fn url(&self, query: &IconQuery) -> Option<String>;

    async fn fetch(&self, fetcher: &IconFetcher, query: &IconQuery) -> Result<IconResult, Error> {
        match self.url(query) {
            Some(url) => fetcher.fetch_image_url(&url).await,
            None => Ok(IconResult::Unsupported),
        }
    }
}
