use crate::{IconQuery, IconSource};

#[derive(Clone)]
pub struct SafeWallet;

#[async_trait::async_trait]
impl IconSource for SafeWallet {
    fn name(&self) -> &'static str {
        "safewallet"
    }

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
}
