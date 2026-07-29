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
    ) -> Result<Option<DiscoveryResult>, Error> {
        let url = self.url(&query).ok_or(Error::Unsupported)?;

        let icon = client.fetch(&url).await?;

        Ok(Some(DiscoveryResult {
            icon: Some(icon),
            metadata: None,
        }))
    }
}
