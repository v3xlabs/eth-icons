use crate::{Error, IconClient, IconQuery, IconResult, IconSource};

pub struct Avara;

impl IconSource for Avara {
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

    async fn fetch(&self, client: &IconClient, query: IconQuery) -> Result<IconResult, Error> {
        let Some(url) = self.url(&query) else {
            return Ok(IconResult::Unsupported);
        };

        client.fetch_image_url(&url).await
    }
}
