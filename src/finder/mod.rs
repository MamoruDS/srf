use std::collections::HashMap;

use async_trait::async_trait;
use erased_serde;

mod config;
mod fs;
// mod traits;

pub trait FindResult: erased_serde::Serialize + std::fmt::Debug {}

#[async_trait]
pub trait Finder: Sync + Send + std::fmt::Debug {
    async fn find(&self, name: &str) -> Vec<Box<dyn FindResult>>;
}

pub fn get_finder_from_yaml(config_fp: &str) -> HashMap<String, Vec<Box<dyn Finder>>> {
    let content = std::fs::read_to_string(config_fp).unwrap();
    let saved: config::Saved = serde_yaml::from_str(&content).unwrap();
    saved
        .routes
        .into_iter()
        .map(|(k, v)| (k, v.into_iter().map(|f| f.instantiate()).collect()))
        .collect()
}
