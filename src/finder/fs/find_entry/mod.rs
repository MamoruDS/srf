use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use regex::Regex;
use tokio::sync::Mutex;

use crate::finder::utils::templated_regex::{build_renames, build_search_regex};
use crate::finder::{FindResult, Finder};

use super::FSFindResult;
use config::{Entry, EntryConfig, EntryFinderConfig};
use utils::build_walkdir;

pub mod config;
mod utils;

#[derive(Clone, Debug)]
struct EntriesCache {
    cached: Arc<Mutex<(Instant, Option<Vec<String>>)>>,
    ttl: Duration,
}

#[derive(Clone, Debug)]
pub struct FSEntryFinder {
    roots: Vec<Entry>,
    parse_pattern: Regex,
    search_template: String,
    renames: Option<HashMap<String, Vec<(Regex, String)>>>,
    _walkdir_cache: EntriesCache,
}

impl FSEntryFinder {
    pub fn new(
        roots: Vec<EntryConfig>,
        parse_pattern: String,
        search_template: String,
        renames: Option<HashMap<String, Vec<(String, String)>>>,
    ) -> Self {
        let roots = roots
            .into_iter()
            .map(|root| match root {
                EntryConfig::Root(root) => Entry {
                    root,
                    options: None,
                },
                EntryConfig::Configured(entry) => entry,
            })
            .collect();
        let parse_pattern = Regex::new(&parse_pattern).unwrap(); // TODO:
        let renames = build_renames(renames, None);
        Self {
            roots,
            parse_pattern,
            search_template,
            renames,
            _walkdir_cache: EntriesCache {
                cached: Arc::new(Mutex::new((Instant::now(), None))),
                ttl: Duration::from_secs(5),
            },
        }
    }

    fn _cache_walkdir(&self) -> Vec<String> {
        let mut cached = vec![];
        for fss_entry in self.roots.iter() {
            for entry in build_walkdir(&fss_entry.root, &fss_entry.options) {
                let entry = entry.unwrap();
                cached.push(entry.path().to_string_lossy().into());
            }
        }
        cached
    }

    fn _find_in(&self, name: &str, entries: &[String]) -> Vec<FSFindResult> {
        let mut founds = vec![];
        let re = build_search_regex(
            &self.parse_pattern,
            &self.search_template,
            &self.renames,
            name,
        );
        for fp in entries.iter() {
            if !re.is_match(&fp) {
                continue;
            }
            let found = FSFindResult {
                path: fp.to_string(),
                filename: Path::new(fp)
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                is_file: Path::new(fp).is_file(),
            };
            println!("{:?}", found); // TODO:
            founds.push(found);
        }
        founds
    }

    async fn _find(&self, name: &str) -> Vec<FSFindResult> {
        let cached = {
            let mut c = self._walkdir_cache.cached.lock().await;
            match c.1 {
                Some(_) => {
                    if c.0.elapsed() > self._walkdir_cache.ttl {
                        c.1 = Some(self._cache_walkdir());
                        c.0 = Instant::now();
                    }
                }
                None => {
                    c.1 = Some(self._cache_walkdir());
                    c.0 = Instant::now();
                }
            };
            c.1.clone()
        };
        self._find_in(name, cached.as_ref().unwrap())
    }
}

#[async_trait]
impl Finder for FSEntryFinder {
    async fn find(&self, name: &str) -> Vec<Box<dyn FindResult>> {
        self._find(name)
            .await
            .into_iter()
            .map(|f| Box::new(f) as Box<dyn FindResult>)
            .collect()
    }
}

impl EntryFinderConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        let entries_finder =
            FSEntryFinder::new(self.roots, self.pattern, self.template, self.renames);
        Box::new(entries_finder)
    }
}
