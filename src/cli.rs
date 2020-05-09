use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg};

pub fn new() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .setting(AppSettings::ArgRequiredElseHelp)
        .args(&[
            Arg::from_usage("[query] 'Query to look for'"),
            Arg::from_usage("-b --bropages 'Query data from http://bropages.org/'")
                .requires("query"),
            Arg::from_usage("-c --cheat 'Query data from https://cheat.sh/'").requires("query"),
            Arg::from_usage("-C --commandlinefu 'Query data from https://www.commandlinefu.com'")
                .requires("query"),
            Arg::from_usage("-e --eg 'Query data from https://github.com/srsudar/eg'")
                .requires("query"),
            Arg::from_usage("-t --tldr 'Query data from https://tldr.sh/'").requires("query"),
            Arg::from_usage(
                "-s --cheatsheets 'Query data from https://github.com/cheat/cheatsheets'",
            )
            .requires("query"),
            Arg::from_usage("-U --update 'Update local databases'")
                .conflicts_with_all(&["b", "c", "C", "e", "t", "x", "query"]),
            Arg::from_usage("-I --init 'Initialize juli databases'"),
        ])
}
