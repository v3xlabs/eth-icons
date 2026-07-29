use std::fmt::{Debug, Display};

use bytes::Bytes;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Icon {
    pub bytes: Bytes,
    pub mime_type: Option<String>,
}

impl Display for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Icon(size: {:?}, mime_type: {})",
            self.bytes.len(),
            self.mime_type.as_ref().unwrap_or(&"unknown".to_string())
        )
    }
}

impl Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Icon(size: {:?}, mime_type: {})",
            self.bytes.len(),
            self.mime_type.as_ref().unwrap_or(&"unknown".to_string())
        )
    }
}
