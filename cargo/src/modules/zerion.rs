use crate::{Error, IconClient, IconQuery, IconResult, IconSource};

pub struct Zerion;

impl IconSource for Zerion {
    fn url(&self, query: &IconQuery) -> Option<String> {
        match query {
            IconQuery::ERC20(_network_id, address) => Some(format!(
                "https://cdn.zerion.io/{}.png",
                address.to_lowercase(),
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
