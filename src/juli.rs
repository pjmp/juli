use bat::{PagingMode, PrettyPrinter};
use clap::crate_name;
use dirs;
use git2::{Error, Repository};

use std::fs;
use std::path::PathBuf;
use std::thread;

use crate::plugins::{cheatsheets, eg, tldr, gitpull};

pub(crate) fn clone_repo(path: &PathBuf, url: &str) -> Option<Result<Repository, Error>> {
    let plugin_path = root_dir().join(path);

    if !plugin_path.is_dir() {
        return Some(Repository::clone(url, &plugin_path));
    }

    None
}

pub(crate) fn root_dir() -> PathBuf {
    let data_local_dir = match dirs::data_local_dir() {
        None => {
            let home_dir = match dirs::home_dir() {
                None => PathBuf::from(env!("HOME")),
                Some(home_dir) => home_dir,
            };

            home_dir.join(".local/share")
        }
        Some(local_dir) => local_dir,
    };

    data_local_dir.join(crate_name!())
}

pub(crate) fn maybe_init() {
    let dir = root_dir();

    if !&dir.is_dir() {
        fs::create_dir_all(&dir)
            .expect(&format!("Unable to create directory: '{}'", &dir.display()))
    }

    let jobs = [tldr::maybe_init, eg::maybe_init, cheatsheets::maybe_init]
        .iter()
        .map(|cb| thread::spawn(move || cb()));

    for job in jobs {
        let _ = job.join();
    }
}

pub(crate) fn update() {
    let mut jobs = vec![];

    let dir = root_dir().read_dir().unwrap();

    dir.for_each(|entry| {
        let path = entry.unwrap();

        jobs.push(thread::spawn(move || {
            // println!("Fetching: {:?}", &path.path().iter().last().unwrap());


            let name = "origin";
            let branch = "master";

            gitpull::run(name, branch, path.path());
        }));
    });

    for j in jobs {
        let _ = j.join();
    }
}

pub(crate) fn render(content: &[u8]) {
    // list of themes
    // dbg!(PrettyPrinter::new().themes().collect::<Vec<&str>>());

    match PrettyPrinter::new()
        .input_from_bytes(content)
        .language("md")
        .paging_mode(PagingMode::QuitIfOneScreen)
        .print()
    {
        Ok(_) => {}
        Err(_) => {
            println!("{}", String::from_utf8_lossy(content));
        }
    }
}

pub(crate) fn wrap_with_code_block(code: &str) -> String {
    format!("\n\n``` bash\n{}\n```\n\n", code)
}
