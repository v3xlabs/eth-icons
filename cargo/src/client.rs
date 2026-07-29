use reqwest::Client;

use crate::{Error, Icon, IconResult};

pub struct IconClient {
    pub client: Client,
}

impl IconClient {
    pub fn from_reqwest(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch_image_url(&self, url: &str) -> Result<IconResult, Error> {
        let response = self.client.get(url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(IconResult::NotFound);
        }

        let response = response.error_for_status()?;

        let mime_type = response
            .headers()
            .get("content-type")
            .and_then(|x| x.to_str().ok().map(|x| x.to_string()));
        let body = response.bytes().await?;

        let icon = Icon {
            bytes: body,
            mime_type,
        };

        Ok(icon.into())
    }
}
