mod cli;
mod juli;
mod plugins;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::new().get_matches();

    juli::maybe_init();

    if args.is_present("update") {
        juli::update();
        return Ok(());
    }

    // we can safely do this because all the other flags requires `query`.
    let query = match args.value_of("query") {
        None => {
            let _ = cli::new().print_help();
            std::process::exit(1)
        }
        Some(q) => q,
    };

    if args.is_present("C") {
        plugins::commandlinefu::exec(query);
        return Ok(());
    }

    if args.is_present("b") {
        plugins::bropages::exec(query);
        return Ok(());
    }

    if args.is_present("c") {
        plugins::cheat_sh::exec(query);
        return Ok(());
    }

    if args.is_present("e") {
        plugins::eg::exec(query);
        return Ok(());
    }

    if args.is_present("t") {
        plugins::tldr::exec(query);
        return Ok(());
    }

    if args.is_present("x") {
        plugins::cheatsheets::exec(query);
        return Ok(());
    }

    Ok(())
}
