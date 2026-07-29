use crate::{
    Error, IconClient, IconQuery,
    discovery::{DiscoveryMechanism, DiscoveryResult},
};

pub struct Avara;

impl DiscoveryMechanism for Avara {
    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::ERC20(network_id, address) => Some(format!(
                "https://token-logos.family.co/asset?id={}:{}",
                network_id,
                address.to_lowercase()
            )),
            _ => None,
        }
    }

    async fn fetch(&self, client: &IconClient, query: IconQuery) -> Result<DiscoveryResult, Error> {
        let Some(url) = self.url(&query) else {
            return Ok(DiscoveryResult::Unsupported);
        };

        client.fetch_image_url(&url).await
    }
}
