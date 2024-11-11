use anyhow::{Context, Result};
use std::path::PathBuf;

use clap::{arg, command, value_parser};
use poet::{find, print, read, replace, unfold};

fn main() -> Result<()> {
    let matches = command!()
        .about("Parses exported topics")
        .arg_required_else_help(true)
        .arg(
            arg!([path] "File path")
                .value_parser(value_parser!(PathBuf))
                .required(true),
        )
        .arg(arg!(-a --all "Print all lines"))
        .arg(arg!(-f --find <PATTERN> "Print lines with a PATTERN"))
        .arg(
            arg!(-r --replace <PATTERN> "Print lines with OLD replaced by NEW")
                .value_delimiter(' ')
                .num_args(2)
                .value_names(["OLD", "NEW"]),
        )
        .arg(
            arg!(-i --input <FILE> "Take optional input file").value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-u --unfold "Unfold results").requires("input"))
        .get_matches();

    let path = matches
        .get_one::<PathBuf>("path")
        .with_context(|| "no path provided")?;

    let lines: Vec<String>;

    if let Some(pattern) = matches.get_one::<String>("find") {
        if matches.get_flag("unfold") {
            let input = matches
                .get_one::<PathBuf>("input")
                .with_context(|| "no input provided")?;
            lines = unfold(find(pattern, read(path)?)?, pattern, read(input)?)?;
        } else {
            lines = find(pattern, read(path)?)?;
        }
    } else if let Some(pattern) = matches.get_many::<String>("replace") {
        let pattern: Vec<_> = pattern.collect();
        lines = replace(pattern[0], pattern[1], read(path)?)?;
    } else if matches.get_one::<bool>("all").is_some() {
        lines = read(path)?;
    } else {
        unreachable!("Prevented by arg_required_else_help and path being required")
    }

    print(lines)
}
