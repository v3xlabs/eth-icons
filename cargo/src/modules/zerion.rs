use crate::{IconQuery, IconSource};

#[derive(Clone)]
pub struct Zerion;

#[async_trait::async_trait]
impl IconSource for Zerion {
    fn name(&self) -> &'static str {
        "zerion"
    }

    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::ERC20(_network_id, address) => Some(format!(
                "https://cdn.zerion.io/{}.png",
                address.to_lowercase(),
            )),
            _ => None,
        }
    }
}
