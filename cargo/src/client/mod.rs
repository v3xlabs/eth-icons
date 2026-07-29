use std::sync::Arc;

use futures_util::future::join_all;

use crate::{IconFetcher, IconQuery, IconSource, result::SourceResult};
use builder::IconClientBuilder;

pub mod builder;

pub struct IconClient {
    fetcher: IconFetcher,
    sources: Vec<Arc<dyn IconSource>>,
}

impl IconClient {
    pub fn new() -> Self {
        Self::builder().with_defaults().build()
    }

    pub fn builder() -> IconClientBuilder {
        IconClientBuilder::new()
    }

    pub fn fetcher(&self) -> &IconFetcher {
        &self.fetcher
    }

    pub fn sources(&self) -> &[Arc<dyn IconSource>] {
        &self.sources
    }

    pub async fn resolve(&self, query: IconQuery) -> Vec<SourceResult> {
        join_all(self.sources.iter().map(|source| async {
            SourceResult {
                source: source.name(),
                result: source.fetch(&self.fetcher, &query).await,
            }
        }))
        .await
    }
}

impl Default for IconClient {
    fn default() -> Self {
        Self::new()
    }
}
