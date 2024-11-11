use anyhow::{Context, Result};
use std::path::PathBuf;

use clap::{arg, command, value_parser};
use poet::{all, find, print};

fn main() -> Result<()> {
    let matches = command!()
        .about("Parses exported topics")
        .arg(arg!(-f --find <PATTERN> "Prints out lines with occurrences of a pattern"))
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
    } else {
        lines = all(path)?;
    }

    print(lines)
}
