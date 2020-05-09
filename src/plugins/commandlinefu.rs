use base64;
use minreq;

use crate::juli;

fn parse(content: &String) -> String {
    let mut fragments = content
        .trim_end()
        .split('\n')
        // skipping ahead first lines that contains this:
        // # commandlinefu.com - questions/comments: danstools00@gmail.com
        .skip(1)
        .collect::<Vec<&str>>();

    if fragments.len() == 0 {
        fragments.insert(0, "# No results");
    }

    fragments
        .iter()
        .fold(String::from("# commandlinefu.com\n\n"), |mut acc, item| {
            if item.len() == 0 {
                return acc;
            }

            if item.starts_with('#') {
                acc.push_str(item);
            } else {
                acc.push_str(juli::wrap_with_code_block(item).as_str());
            }

            acc
        })
}

fn fetch(query: &str) -> String {
    let url = format!(
        "https://www.commandlinefu.com/commands/matching/{}/{}/plaintext",
        query,
        base64::encode(query)
    );

    match minreq::get(url).send() {
        Ok(response) => match response.as_str() {
            Ok(r) => r.to_string(),
            Err(err) => format!("\n\nError: {}\n", err.to_string()),
        },
        Err(err) => format!("\n\nError: {}\n", err.to_string()),
    }
}

pub(crate) fn exec(query: &str) {
    let content = parse(&fetch(query));

    juli::render(content.as_bytes());
}
