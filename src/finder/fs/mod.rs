use serde::Serialize;

use super::FindResult;

pub mod config;
mod find_entry;

#[derive(Debug, Serialize)]
pub struct FSFindResult {
    pub path: String,
    pub filename: String,
    pub is_file: bool,
}

impl FindResult for FSFindResult {}
