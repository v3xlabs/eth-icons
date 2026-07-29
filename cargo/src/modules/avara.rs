use crate::{
    IconClient,
    Error,
    IconQuery,
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

    async fn fetch(
        &self,
        client: &IconClient,
        query: IconQuery,
    ) -> Result<Option<DiscoveryResult>, Error> {
        let url = self.url(&query).ok_or(Error::Unsupported)?;

        let icon = client.fetch(&url).await?;

        Ok(Some(DiscoveryResult {
            icon: Some(icon),
            metadata: None,
        }))
    }
}
