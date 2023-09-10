use std::collections::HashMap;
use std::fmt::Debug;

use async_trait::async_trait;
use erased_serde;
use serde::{Serialize, Serializer};

mod config;
mod fs;
mod utils;
// mod traits;

pub trait FindResult: erased_serde::Serialize + Debug {}

// TODO:
impl Serialize for Box<dyn FindResult> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        erased_serde::serialize(self.as_ref(), serializer)
    }
}

#[async_trait]
pub trait Finder: Sync + Send + Debug {
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
