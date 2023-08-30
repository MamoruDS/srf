use serde::Deserialize;

use crate::finder::Finder;

use super::find_entry::config::EntryFinderConfig;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum FsFinderConfig {
    #[serde(rename = "find_entry")]
    FindEntry(EntryFinderConfig),
}

impl FsFinderConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        match self {
            FsFinderConfig::FindEntry(config) => config.instantiate(),
        }
    }
}
