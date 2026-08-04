use std::sync::Arc;

use crate::{
    IconClient, IconFetcher, IconSource,
    modules::{Avara, Blockscout, SafeWallet, Smoldapp, Zerion},
};

#[derive(Default)]
pub struct IconClientBuilder {
    client: Option<reqwest::Client>,
    max_response_bytes: Option<usize>,
    sources: Vec<Arc<dyn IconSource>>,
}

impl IconClientBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_reqwest(mut self, client: reqwest::Client) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_max_response_bytes(mut self, max_response_bytes: usize) -> Self {
        self.max_response_bytes = Some(max_response_bytes);
        self
    }

    pub fn with_source<S: IconSource + 'static>(mut self, source: S) -> Self {
        self.sources.push(Arc::new(source));
        self
    }

    pub fn with_shared_source(mut self, source: Arc<dyn IconSource>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn with_defaults(self) -> Self {
        self.with_source(Blockscout)
            .with_source(Avara)
            .with_source(Zerion)
            .with_source(Smoldapp)
            .with_source(SafeWallet)
    }

    pub fn build(self) -> IconClient {
        IconClient {
            fetcher: IconFetcher::new(self.client, self.max_response_bytes),
            sources: self.sources,
        }
    }
}
