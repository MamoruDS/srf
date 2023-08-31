use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant};

use async_trait::async_trait;
use regex::{Captures, Regex};
use tokio::sync::Mutex;

use crate::finder::{FindResult, Finder};

use super::FSFindResult;
use config::{Entry, EntryConfig, EntryFinderConfig};
use utils::{build_renames, build_walkdir};

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
        let parse_pattern = Regex::new(&parse_pattern).unwrap();
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

    fn build_search_pattern(&self, input: &str) -> Regex {
        let template = match self.renames.as_ref() {
            Some(renames) => {
                let caps = self.parse_pattern.captures(input).unwrap();
                let extendable_re = Regex::new(r"\$((?P<var>\w+)|\{(?P<braced>\w+)\})").unwrap();
                let template =
                    extendable_re.replace_all(&self.search_template, |matches: &Captures| {
                        let var = matches
                            .name("var")
                            .or_else(|| matches.name("braced"))
                            .unwrap()
                            .as_str();
                        let caps_val = caps.name(var);
                        let placer = match (caps_val, renames.get(var)) {
                            (Some(val), Some(cases)) => {
                                let mut placer = None;
                                for (re, tmp) in cases.into_iter() {
                                    if re.is_match(val.as_str()) {
                                        placer =
                                            Some(re.replace_all(val.as_str(), tmp).to_string());
                                    }
                                }
                                placer
                            }
                            _ => None,
                        };
                        let placer = placer.unwrap_or(format!(r"${{{}}}", var));
                        placer
                    });
                Some(template)
            }
            _ => None,
        };
        Regex::new(&match template {
            Some(t) => self.parse_pattern.replace_all(input, t),
            None => self.parse_pattern.replace_all(input, &self.search_template),
        })
        .unwrap()
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
        let re = self.build_search_pattern(name);
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
        let mut cached = self._walkdir_cache.cached.lock().await;
        match cached.1 {
            Some(_) => {
                if cached.0.elapsed() > self._walkdir_cache.ttl {
                    cached.1 = Some(self._cache_walkdir());
                    cached.0 = Instant::now();
                }
            }
            None => {
                cached.1 = Some(self._cache_walkdir());
                cached.0 = Instant::now();
            }
        };
        self._find_in(name, cached.1.as_ref().unwrap())
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
