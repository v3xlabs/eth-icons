use crate::{IconQuery, IconSource};

/// [source](https://tokens.smold.app/ethereum)
#[derive(Clone)]
pub struct Smoldapp;

#[async_trait::async_trait]
impl IconSource for Smoldapp {
    fn name(&self) -> &'static str {
        "smoldapp"
    }

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
}
