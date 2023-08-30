use super::config::WalkdirOptions;
use regex::Regex;
use std::collections::HashMap;
use walkdir::WalkDir;

pub fn build_renames(
    renames: Option<HashMap<String, Vec<(String, String)>>>,
    valid_vars: Option<&Vec<&str>>,
) -> Option<HashMap<String, Vec<(Regex, String)>>> {
    if renames.as_ref().is_none() {
        return None;
    }
    if renames.as_ref().unwrap().is_empty() {
        return None;
    }
    let mut renames_re = HashMap::new();
    for (name, cases) in renames.unwrap().into_iter() {
        if valid_vars.is_some() && !valid_vars.unwrap().contains(&name.as_str()) {
            continue;
        }
        let mut cases_re = vec![];
        for (pattern, template) in cases.into_iter() {
            let re = Regex::new(&pattern).unwrap();
            cases_re.push((re, template));
        }
        renames_re.insert(name, cases_re);
    }
    Some(renames_re)
}

pub fn build_walkdir(root: &str, options: &Option<WalkdirOptions>) -> WalkDir {
    let mut walkdir = WalkDir::new(root);
    if let Some(options) = options.as_ref() {
        if let Some(depth) = options.max_depth {
            walkdir = walkdir.max_depth(depth);
        }
        if let Some(follow) = options.follow_links {
            walkdir = walkdir.follow_links(follow);
        }
    }
    walkdir
}
