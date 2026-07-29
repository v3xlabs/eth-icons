use crate::{Error, IconClient, IconQuery, IconResult, IconSource};

pub struct SafeWallet;

impl IconSource for SafeWallet {
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

    async fn fetch(&self, client: &IconClient, query: IconQuery) -> Result<IconResult, Error> {
        let Some(url) = self.url(&query) else {
            return Ok(IconResult::Unsupported);
        };

        client.fetch_image_url(&url).await
    }
}
