use serde::Deserialize;

use crate::finder::Finder;

use super::file::config::TextFileFinderConfig;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TextFinderConfig {
    #[serde(rename = "file")]
    TextFileEntry(TextFileFinderConfig),
}

impl TextFinderConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        match self {
            TextFinderConfig::TextFileEntry(config) => config.instantiate(),
        }
    }
}
