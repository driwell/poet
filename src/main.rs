use std::path::PathBuf;

use clap::{arg, command, value_parser};

fn main() {
    let matches = command!()
        .about("Parses exported topics")
        .arg(arg!(-c --clean <FILE> "Prints out a clean file").value_parser(value_parser!(PathBuf)))
        .get_matches();

    println!(
        "file: {}",
        matches.get_one::<PathBuf>("clean").unwrap().display()
    );
}
