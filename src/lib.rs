use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Lines};

use anyhow::Result;
use std::path::Path;

pub fn find(pattern: &str, path: &Path) -> Result<()> {
    let lines = read_lines(path)?;

    for line in lines.map_while(Result::ok) {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn print(path: &Path) -> Result<()> {
    let lines = read_lines(path)?;

    for line in lines.map_while(Result::ok) {
        println!("{}", line);
    }

    Ok(())
}

fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}
