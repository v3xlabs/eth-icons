use crate::{
    IconClient, Error, IconQuery,
    discovery::{DiscoveryMechanism, DiscoveryResult},
};

pub struct Zerion;

impl DiscoveryMechanism for Zerion {
    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::ERC20(_network_id, address) => Some(format!(
                "https://cdn.zerion.io/{}.png",
                address.to_lowercase(),
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
