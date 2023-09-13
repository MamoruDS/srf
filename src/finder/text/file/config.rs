use std::collections::HashMap;

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct TextFileFinderConfig {
    pub files: Vec<String>,
    pub pattern: String,
    pub template: String,
    pub renames: Option<HashMap<String, Vec<(String, String)>>>,
}
