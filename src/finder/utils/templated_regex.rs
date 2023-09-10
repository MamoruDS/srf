use std::collections::HashMap;

use regex::{Captures, Regex};

pub fn build_search_regex(
    parse_pattern: &Regex,
    search_template: &str,
    renames: &Option<HashMap<String, Vec<(Regex, String)>>>,
    input: &str,
) -> Regex {
    let template = match renames.as_ref() {
        Some(renames) => {
            let caps = parse_pattern.captures(input).unwrap();
            let extendable_re = Regex::new(r"\$((?P<var>\w+)|\{(?P<braced>\w+)\})").unwrap();
            let template = extendable_re.replace_all(search_template, |matches: &Captures| {
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
                                placer = Some(re.replace_all(val.as_str(), tmp).to_string());
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
        Some(t) => parse_pattern.replace_all(input, t),
        None => parse_pattern.replace_all(input, search_template),
    })
    .unwrap()
}
