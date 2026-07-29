use reqwest::{Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::{Error, Icon, IconResult};

pub struct IconFetcher {
    pub client: reqwest::Client,
}

impl IconFetcher {
    pub fn new(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub async fn fetch_image_url(&self, url: &str) -> Result<IconResult, Error> {
        let Some(response) = self.get(url).await? else {
            return Ok(IconResult::NotFound);
        };

        let mime_type = response
            .headers()
            .get("content-type")
            .and_then(|x| x.to_str().ok().map(|x| x.to_string()));
        let bytes = response.bytes().await?;

        Ok(Icon { bytes, mime_type }.into())
    }

    pub async fn fetch_json<T: DeserializeOwned>(&self, url: &str) -> Result<Option<T>, Error> {
        let Some(response) = self.get(url).await? else {
            return Ok(None);
        };

        Ok(Some(response.json().await?))
    }

    async fn get(&self, url: &str) -> Result<Option<Response>, Error> {
        let response = self.client.get(url).send().await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        Ok(Some(response.error_for_status()?))
    }
}

impl Default for IconFetcher {
    fn default() -> Self {
        Self::new(reqwest::Client::new())
    }
}
