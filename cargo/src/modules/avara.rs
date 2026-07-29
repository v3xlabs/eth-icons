use crate::{
    discovery::{DiscoveryError, DiscoveryMechanism, DiscoveryQuery, DiscoveryResult}, identity::{Address, NetworkId},
};

pub struct AvaraIconModule;

fn erc20_url(network_id: NetworkId, address: Address) -> String {
    format!(
        "https://token-logos.family.co/asset?id={}:{}",
        network_id, address.to_lowercase()
    )
}

impl DiscoveryMechanism for AvaraIconModule {
    async fn discover(&self, query: DiscoveryQuery) -> Result<Option<DiscoveryResult>, DiscoveryError> {
        match query {
            DiscoveryQuery::ERC20(network_id, address) => {
                let url = erc20_url(network_id, address);
                let response = reqwest::get(url).await.map_err(DiscoveryError::HttpError)?;
                let body = response.text().await.map_err(DiscoveryError::HttpError)?;
                let result = DiscoveryResult {
                    icon: Some(body),
                    metadata: None,
                };
                Ok(Some(result))
            }
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_discover() {
        let module = AvaraIconModule;
        let query = DiscoveryQuery::ERC20(1, Address("0x0000000000000000000000000000000000000000".to_string()));
        let result = module.discover(query).await.unwrap().unwrap();
        println!("{:?}", result);
        assert!(result.icon.is_some());
    }
}
