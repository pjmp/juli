use std::fs;

use crate::juli;

// TODO: parse results to make the render consistent.

// fn parse() -> String {}

fn fetch(query: &str) -> String {
    let root = juli::root_dir().join("eg/eg/examples/");
    let dirs = fs::read_dir(&root).unwrap();

    let mut path_result = None;

    dirs.for_each(|item| {
        let item = item.unwrap();

        if item.path().ends_with(format!("{}.md", query)) {
            path_result = Some(item);
            return;
        }
    });

    match path_result {
        None => format!("# No results found for: '{}'", query),
        Some(path) => fs::read_to_string(path.path()).unwrap(),
    }
}

pub(crate) fn maybe_init() {
    let eg_root = juli::root_dir().join("eg");

    if !eg_root.is_dir() {
        juli::clone_repo(&eg_root, "https://github.com/srsudar/eg.git");
    }
}

pub(crate) fn exec(query: &str) {
    let content = fetch(query);

    juli::render(content.as_bytes());
}
