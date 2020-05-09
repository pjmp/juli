use crate::juli;
use std::fs;

const NAME: &str = "tldr";

fn parse(content: &String) -> String {
    let mut formatted = format!("# {}\n\n", NAME);

    for curr in content.split('\n') {
        let curr = curr.replace("{{", "");
        let curr = curr.replace("}}", "");

        if curr.starts_with(">") {
            formatted.push_str(format!("{}\n\n", curr.replacen(">", "##", 1)).as_str());
        }

        if curr.starts_with("-") {
            formatted.push_str(format!("{}", curr.replacen("-", "###", 1)).as_str());
        }

        if curr.starts_with("`") && curr.ends_with("`") {
            formatted.push_str(juli::wrap_with_code_block(&curr[1..curr.len() - 1]).as_str());
        }
    }

    format!("{}", formatted.trim_end())
}

fn fetch(query: &str) -> String {
    let root = juli::root_dir().join(NAME);

    let dirs = match fs::read_dir(root.join("pages")) {
        Ok(d) => d,
        Err(err) => return format!("> Error: {}\n> Try running `juli --init`", err.to_string()),
    };

    let mut path_result = None;

    dirs.for_each(|dir| {
        let inner = fs::read_dir(dir.unwrap().path()).unwrap();

        inner.for_each(|item| {
            let item = item.unwrap();

            let in_common = item.path().ends_with(format!("common/{}.md", query));
            let in_os = item
                .path()
                .ends_with(format!("{}/{}.md", std::env::consts::OS, query));

            if in_common || in_os {
                path_result = Some(item);
                return;
            }
        });
    });

    match path_result {
        None => format!("> No results found for: '{}'", query),
        Some(path) => fs::read_to_string(path.path()).unwrap(),
    }
}

pub(crate) fn maybe_init() {
    let tldr_root = juli::root_dir().join(NAME);

    if !tldr_root.is_dir() {
        if let Some(clone_response) =
            juli::clone_repo(&tldr_root, "https://github.com/tldr-pages/tldr.git")
        {
            match clone_response {
                Ok(_) => {}
                Err(err) => {
                    dbg!(err);
                }
            }
        }
    }
}

pub(crate) fn exec(query: &str) {
    let content = parse(&fetch(query));

    juli::render(content.as_bytes());
}
