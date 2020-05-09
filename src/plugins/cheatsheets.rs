use std::fs;

use crate::juli;

fn parse(content: &String) -> String {
    format!("# cheatsheets\n\n{}", content)
}

fn fetch(query: &str) -> String {
    let dirs = fs::read_dir(&juli::root_dir().join("cheatsheets")).unwrap();

    let mut path_result = None;

    dirs.for_each(|item| {
        let item = item.unwrap();

        if item.path().ends_with(format!("{}", query)) {
            path_result = Some(item);
            return;
        }
    });

    match path_result {
        None => format!("> No results found for: '{}'", query),
        Some(path) => fs::read_to_string(path.path()).unwrap(),
    }
}

pub(crate) fn maybe_init() {
    let cheatsheets_root = juli::root_dir().join("cheatsheets");

    if !cheatsheets_root.is_dir() {
        juli::clone_repo(
            &cheatsheets_root,
            "https://github.com/cheat/cheatsheets.git",
        );
    }
}

pub(crate) fn exec(query: &str) {
    let content = parse(&fetch(query));

    juli::render(content.as_bytes());
}
