use serde::Deserialize;

use crate::finder::Finder;

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum TextFinderConfig {}

impl TextFinderConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        unimplemented!()
    }
}
