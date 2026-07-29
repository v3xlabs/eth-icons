use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Error, IconClient, IconQuery,
    discovery::{DiscoveryMechanism, DiscoveryResult},
};

pub struct Blockscout;

impl DiscoveryMechanism for Blockscout {
    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::ERC20(network_id, address) => {
                if *network_id != 1 {
                    return None;
                }
                Some(format!(
                    "https://eth.blockscout.com/api/v2/tokens/{}",
                    address.to_lowercase()
                ))
            }
            _ => None,
        }
    }

    async fn fetch(
        &self,
        client: &IconClient,
        query: IconQuery,
    ) -> Result<Option<DiscoveryResult>, Error> {
        let url = self.url(&query).ok_or(Error::Unsupported)?;

        let response = client
            .client
            .get(url)
            .send()
            .await
            .map_err(Error::HttpError)?;

        let status = response.status();
        if !status.is_success() {
            return Err(Error::HttpError(
                response.error_for_status().unwrap_err(),
            ));
        }

        let body: BlockscoutAssetMetadata =
            response.json().await.map_err(Error::HttpError)?;
        let icon_url = body.icon_url.ok_or(Error::NotFound)?;

        let icon = client.fetch_image_url(&icon_url).await?;

        Ok(Some(DiscoveryResult {
            icon: Some(icon),
            metadata: None,
        }))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockscoutAssetMetadata {
    pub icon_url: Option<String>,
    // pub decimals: String,
    // pub name: String,
    // pub symbol: String,
    #[serde(flatten)]
    pub other: HashMap<String, String>,
}
