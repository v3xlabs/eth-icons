use crate::{
    Error, IconClient, IconQuery,
    discovery::{DiscoveryMechanism, DiscoveryResult},
};

pub struct SafeWallet;

impl DiscoveryMechanism for SafeWallet {
    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::Network(network_id) => Some(format!(
                "https://safe-transaction-assets.safe.global/chains/{}/chain_logo.png",
                network_id
            )),
            IconQuery::Native(network_id) => Some(format!(
                "https://safe-transaction-assets.safe.global/chains/{}/currency_logo.png",
                network_id
            )),
            _ => None,
        }
    }

    async fn fetch(
        &self,
        client: &IconClient,
        query: IconQuery,
    ) -> Result<DiscoveryResult, Error> {
        let Some(url) = self.url(&query) else {
            return Ok(DiscoveryResult::Unsupported);
        };

        client.fetch_image_url(&url).await
    }
}
