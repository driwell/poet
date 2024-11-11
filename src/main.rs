use anyhow::{Context, Result};
use std::path::PathBuf;

use clap::{arg, command, value_parser};
use poet::{all, find, print, replace};

fn main() -> Result<()> {
    let matches = command!()
        .about("Parses exported topics")
        .arg(arg!(-a --all "Print all lines"))
        .arg(arg!(-f --find <PATTERN> "Print lines with a PATTERN"))
        .arg(
            arg!([path] "File path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .arg(
            arg!(-r --replace <PATTERN> "Print lines with OLD replaced by NEW")
                .value_delimiter(' ')
                .num_args(2)
                .value_names(["OLD", "NEW"]),
        )
        .get_matches();

    let path = matches
        .get_one::<PathBuf>("path")
        .with_context(|| "no path provided")?;

    let lines: Vec<String>;

    if let Some(pattern) = matches.get_one::<String>("find") {
        lines = find(pattern, path)?;
    } else if let Some(pattern) = matches.get_many::<String>("replace") {
        let pattern: Vec<_> = pattern.collect();
        lines = replace(pattern[0], pattern[1], path)?;
    } else if matches.get_one::<String>("all").is_some() {
        lines = all(path)?;
    } else {
        // TODO: handle these cases, probably use subcommands and make option required
        panic!("shouln't be able to pass no options")
    }

    print(lines)
}
