use crate::{Error, IconClient, IconQuery, IconResult, IconSource};

/// https://tokens.smold.app/ethereum
pub struct Smoldapp;

impl IconSource for Smoldapp {
    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::Network(network_id) => Some(format!(
                "https://raw.githubusercontent.com/smoldapp/tokenassets/main/chains/{}/logo.svg",
                network_id
            )),
            IconQuery::Native(network_id) => {
                if *network_id != 1 {
                    return None;
                }
                Some("https://raw.githubusercontent.com/smoldapp/tokenassets/main/tokens/1/0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee/logo.svg".to_string())
            }
            IconQuery::ERC20(network_id, address) => Some(format!(
                "https://raw.githubusercontent.com/smoldapp/tokenassets/main/tokens/{}/{}/logo.svg",
                network_id,
                address.to_lowercase()
            )),
        }
    }

    async fn fetch(&self, client: &IconClient, query: IconQuery) -> Result<IconResult, Error> {
        let Some(url) = self.url(&query) else {
            return Ok(IconResult::Unsupported);
        };

        client.fetch_image_url(&url).await
    }
}
