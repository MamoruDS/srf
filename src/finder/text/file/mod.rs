use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use async_trait::async_trait;
use regex::Regex;
use serde::Serialize;

use crate::finder::utils::templated_regex::{build_renames, build_search_regex};
use crate::finder::{FindResult, Finder};

use config::TextFileFinderConfig;

pub mod config;

#[derive(Debug, Serialize)]
struct TextFindResult {
    filepath: String,
    lineno: usize,
    line: String,
}

impl FindResult for TextFindResult {}

#[derive(Clone, Debug)]
pub struct TextFileFinder {
    files: Vec<String>,
    parse_pattern: Regex,
    search_template: String,
    renames: Option<HashMap<String, Vec<(Regex, String)>>>,
}

impl TextFileFinder {
    pub fn new(
        files: Vec<String>,
        parse_pattern: String,
        search_template: String,
        renames: Option<HashMap<String, Vec<(String, String)>>>,
    ) -> Self {
        let parse_pattern = Regex::new(&parse_pattern).unwrap(); // TODO:
        let renames = build_renames(renames, None);
        Self {
            files,
            parse_pattern,
            search_template,
            renames,
        }
    }

    fn _find_in<'a>(&self, name: &str, lines: &'a [String]) -> Vec<(usize, &'a String)> {
        let mut founds = vec![];
        let re = build_search_regex(
            &self.parse_pattern,
            &self.search_template,
            &self.renames,
            name,
        );
        for (lineno, line) in lines.iter().enumerate() {
            if re.is_match(line) {
                founds.push((lineno + 1, line));
            }
        }
        founds
    }

    fn _find(&self, name: &str) -> Vec<TextFindResult> {
        let mut results = vec![];
        for fp in self.files.iter() {
            let file = File::open(fp).unwrap(); // TODO:
            let lines = BufReader::new(file)
                .lines()
                .map(|l| l.unwrap())
                .collect::<Vec<_>>();
            let founds = self._find_in(name, &lines);
            results.extend(founds.into_iter().map(|(lineno, line)| TextFindResult {
                filepath: fp.clone(),
                lineno,
                line: line.clone(),
            }));
        }
        results
    }
}

#[async_trait]
impl Finder for TextFileFinder {
    async fn find(&self, name: &str) -> Vec<Box<dyn FindResult>> {
        self._find(name)
            .into_iter()
            .map(|f| Box::new(f) as Box<dyn FindResult>)
            .collect()
    }
}

impl TextFileFinderConfig {
    pub fn instantiate(self) -> Box<dyn Finder> {
        Box::new(TextFileFinder::new(
            self.files,
            self.pattern,
            self.template,
            self.renames,
        ))
    }
}
