use std::path::PathBuf;

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .about("Parses exported topics")
        .arg(arg!(-f --find <PATTERN> "Prints out lines with occurrences of a pattern"))
        .arg(
            arg!([path] "File path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .get_matches();
}
