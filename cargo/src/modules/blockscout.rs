use crate::discovery::{DiscoveryMechanism, DiscoveryQuery, DiscoveryResults};

pub struct BlockscoutIconModule;

impl DiscoveryMechanism for AvaraIconModule {
    async fn discover(&self, query: DiscoveryQuery) -> DiscoveryResults {
        let results = vec![];
        
        
        DiscoveryResults {
            query,
            results
        }
    }
}
