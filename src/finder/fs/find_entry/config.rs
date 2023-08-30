use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct WalkdirOptions {
    pub follow_links: Option<bool>,
    pub max_depth: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Entry {
    pub root: String,
    pub options: Option<WalkdirOptions>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum EntryConfig {
    Root(String),
    Configured(Entry),
}

#[derive(Clone, Debug, Deserialize)]
pub struct FindEntryConfig {
    pub roots: Vec<EntryConfig>,
    pub pattern: String,
    pub template: String,
    pub renames: Option<HashMap<String, Vec<(String, String)>>>,
}
