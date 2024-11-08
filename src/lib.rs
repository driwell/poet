use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Lines},
    path::Path,
};

pub fn find(pattern: &str, path: &Path) -> Result<()> {
    let lines = read_lines(path)?;

    for line in lines
        .map_while(Result::ok)
        .filter(|line| line.contains(pattern))
    {
        println!("{}", line);
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
