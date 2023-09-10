use walkdir::WalkDir;

use super::config::WalkdirOptions;

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
