use reqwest::Client;

use crate::{Error, Icon};

pub struct IconClient {
    pub client: Client,
}

impl IconClient {
    pub fn from_reqwest(client: Client) -> Self {
        Self { client }
    }

    pub async fn fetch(&self, url: &str) -> Result<Icon, Error> {
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(Error::HttpError)?;
        let status = response.status();

        if !status.is_success() {
            return Err(Error::HttpError(response.error_for_status().unwrap_err()));
        }

        let mime_type = response
            .headers()
            .get("content-type")
            .map(|h| h.to_str().unwrap_or("image/png"))
            .unwrap_or("image/png")
            .to_string();
        let body = response.bytes().await.map_err(Error::HttpError)?;
        let icon = Icon {
            bytes: body,
            mime_type,
        };

        Ok(icon)
    }
}
