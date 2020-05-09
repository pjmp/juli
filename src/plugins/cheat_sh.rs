use minreq;

use crate::juli;

fn parse(content: &String) -> String {
    format!("# cheat.sh\n\n{}", content)
}

fn fetch(query: &str) -> String {
    let url = format!("https://cheat.sh/{}?T", query);

    // `cheat.sh` will send full html if user agent is anything other than curl
    let client = minreq::get(url).with_header("User-Agent", "curl/7.69.1");

    match client.send() {
        Ok(response) => match response.as_str() {
            Ok(res) => res.to_string(),
            Err(err) => format!("Error: {}", err.to_string()),
        },
        Err(err) => format!("Error: {}", err.to_string()),
    }
}

pub(crate) fn exec(query: &str) {
    let content = parse(&fetch(query));

    juli::render(content.as_bytes());
}
