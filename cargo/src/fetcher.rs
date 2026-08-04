use bytes::{Bytes, BytesMut};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;

use crate::{Error, Icon, IconResult};

pub const MAX_RESPONSE_BYTES: usize = 1024 * 1024;

pub struct IconFetcher {
    pub client: Client,
    pub max_response_bytes: usize,
}

impl Default for IconFetcher {
    fn default() -> Self {
        Self::new(None, None)
    }
}

impl IconFetcher {    
    pub fn new(client: Option<Client>, max_response_bytes: Option<usize>) -> Self {
        Self {
            client: client.unwrap_or_default(),
            max_response_bytes: max_response_bytes.unwrap_or(MAX_RESPONSE_BYTES),
        }
    }

    pub fn with_max_response_bytes(mut self, max_response_bytes: usize) -> Self {
        self.max_response_bytes = max_response_bytes;
        self
    }

    pub async fn fetch_image_url(&self, url: &str) -> Result<IconResult, Error> {
        let Some(response) = self.get(url).await? else {
            return Ok(IconResult::NotFound);
        };

        let mime_type = response
            .headers()
            .get("content-type")
            .and_then(|x| x.to_str().ok().map(|x| x.to_string()));
        let bytes = self.read_response(response).await?;

        Ok(Icon { bytes, mime_type }.into())
    }

    pub async fn fetch_json<T: DeserializeOwned>(&self, url: &str) -> Result<Option<T>, Error> {
        let Some(response) = self.get(url).await? else {
            return Ok(None);
        };

        let bytes = self.read_response(response).await?;

        Ok(Some(serde_json::from_slice(&bytes)?))
    }

    async fn get(&self, url: &str) -> Result<Option<Response>, Error> {
        let response = self.client.get(url).send().await?;

        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }

        Ok(Some(response.error_for_status()?))
    }

    async fn read_response(&self, mut response: Response) -> Result<Bytes, Error> {
        if let (max_bytes, Some(content_length)) =
            (self.max_response_bytes, response.content_length())
            && content_length > max_bytes as u64
        {
            return Err(Error::ResponseTooLarge { max_bytes });
        }

        let mut bytes = BytesMut::new();
        while let Some(chunk) = response.chunk().await? {
            if let max_bytes = self.max_response_bytes
                && chunk.len() > max_bytes.saturating_sub(bytes.len())
            {
                return Err(Error::ResponseTooLarge { max_bytes });
            }
            bytes.extend_from_slice(&chunk);
        }

        Ok(bytes.freeze())
    }
}
