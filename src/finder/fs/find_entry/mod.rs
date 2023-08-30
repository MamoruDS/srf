use std::collections::HashMap;

use regex::{Captures, Regex};

use crate::finder::{FindResult, Finder};

use super::FSFindResult;
use config::{Entry, EntryConfig, FindEntryConfig};
use utils::{build_renames, build_walkdir};

pub mod config;
mod utils;

#[derive(Clone, Debug)]
pub struct FindEntry {
    roots: Vec<Entry>,
    parse_pattern: Regex,
    search_template: String,
    renames: Option<HashMap<String, Vec<(Regex, String)>>>,
}

impl FindEntry {
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
}

impl Finder for FindEntry {
    fn find(&self, name: &str) -> Vec<Box<dyn FindResult>> {
        let mut founds = vec![];
        let re = self.build_search_pattern(name);
        for fss_entry in self.roots.iter() {
            for entry in build_walkdir(&fss_entry.root, &fss_entry.options) {
                let entry = entry.unwrap();
                let fp = entry.path().to_string_lossy();
                if !re.is_match(&fp) {
                    continue;
                }
                let found = FSFindResult {
                    path: fp.to_string(),
                    filename: entry.file_name().to_string_lossy().to_string(),
                    is_file: entry.path().is_file(),
                };
                println!("{}", fp); // TODO:
                founds.push(Box::new(found) as Box<dyn FindResult>);
            }
        }
        founds
    }
}

impl FindEntryConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        let entries_finder = FindEntry::new(self.roots, self.pattern, self.template, self.renames);
        Box::new(entries_finder)
    }
}
