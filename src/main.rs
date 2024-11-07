use anyhow::{Context, Result};
use std::path::PathBuf;

use clap::{arg, command, value_parser};
use poet::find;

fn main() -> Result<()> {
    let matches = command!()
        .about("Parses exported topics")
        .arg(arg!(-f --find <PATTERN> "Prints out lines with occurrences of a pattern"))
        .arg(
            arg!([path] "File path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .get_matches();

    find(
        matches
            .get_one::<String>("find")
            .with_context(|| "no pattern provided")?,
        matches
            .get_one::<PathBuf>("path")
            .with_context(|| "no path provided")?,
    )
}
