use minreq;
use serde::Deserialize;

use crate::juli;

#[derive(Deserialize, Debug)]
struct BroResponse {
    msg: String,
}

fn parse(content: Vec<BroResponse>) -> String {
    let formatted = content
        .iter()
        .fold(String::from("# bropages\n"), |mut outer_acc, curr| {
            let block: Vec<&str> = curr.msg.split('\n').collect();

            let line = block.iter().fold("".to_string(), |mut inner_acc, item| {
                if item.len() == 0 {
                    return inner_acc;
                }

                if item.starts_with("#") {
                    inner_acc.push_str(format!("\n{}", item.replacen("#", "##", 1)).as_str());
                } else {
                    inner_acc.push_str(juli::wrap_with_code_block(item).as_str());
                }

                inner_acc
            });

            outer_acc.push_str(line.as_str());

            outer_acc
        });

    format!("{}", formatted.trim_end())
}

fn fetch(query: &str) -> Vec<BroResponse> {
    let url = format!("http://bropages.org/{}.json", query);

    match minreq::get(url).send() {
        Ok(res) => {
            if res.status_code == 404 {
                return vec![BroResponse {
                    msg: format!("# No results found for: '{}'", query),
                }];
            }

            match res.json::<Vec<BroResponse>>() {
                Ok(response) => response,
                Err(err) => vec![BroResponse {
                    msg: format!("# Error: {}", err.to_string()),
                }],
            }
        }
        Err(err) => vec![BroResponse {
            msg: format!("# Error: {}", err.to_string()),
        }],
    }
}

pub(crate) fn exec(query: &str) {
    let content = parse(fetch(query));

    juli::render(content.as_bytes());
}
