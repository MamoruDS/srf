use std::collections::HashMap;

use serde::Deserialize;

use super::fs::config::FsFinderConfig;
use super::text::config::TextFinderConfig;
use super::Finder;

#[derive(Debug, Deserialize)]
pub struct Saved {
    pub routes: HashMap<String, Vec<FinderConfig>>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "finder")]
pub enum FinderConfig {
    #[serde(rename = "fs")]
    Fs(FsFinderConfig),
    #[serde(rename = "text")]
    Text(TextFinderConfig),
}

impl FinderConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        match self {
            FinderConfig::Fs(config) => config.instantiate(),
            FinderConfig::Text(config) => config.instantiate(),
        }
    }
}
