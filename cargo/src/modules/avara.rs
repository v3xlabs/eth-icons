use crate::{IconQuery, IconSource};

#[derive(Clone)]
pub struct Avara;

#[async_trait::async_trait]
impl IconSource for Avara {
    fn name(&self) -> &'static str {
        "avara"
    }

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
}
