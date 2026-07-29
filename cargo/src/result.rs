use crate::{Error, Icon};

#[derive(Debug, Clone)]
pub enum IconResult {
    Found(Icon),
    NotFound,
    Unsupported,
}

impl From<Icon> for IconResult {
    fn from(icon: Icon) -> Self {
        IconResult::Found(icon)
    }
}

impl From<Option<Icon>> for IconResult {
    fn from(icon: Option<Icon>) -> Self {
        match icon {
            Some(icon) => IconResult::Found(icon),
            None => IconResult::NotFound,
        }
    }
}

#[derive(Debug)]
pub struct SourceResult {
    pub source: &'static str,
    pub result: Result<IconResult, Error>,
}
