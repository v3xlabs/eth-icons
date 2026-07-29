pub type NetworkId = u64;

// Ethereum eip-55 Address
#[derive(Debug, Clone)]
pub struct Address(pub String);

impl Address {
    pub fn to_lowercase(&self) -> String {
        self.0.to_lowercase()
    }

    pub fn to_uppercase(&self) -> String {
        self.0.to_uppercase()
    }

    pub fn to_checksummed(&self) -> String {
        // TODO: Implement checksummed address
        // self.0.to_checksummed()
        self.0.clone()
    }
}