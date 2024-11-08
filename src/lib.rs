use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Lines},
    path::Path,
};

pub fn find(pattern: &str, path: &Path) -> Result<Vec<String>> {
    let content = read_lines(path)?;
    let mut lines = Vec::new();

    for line in content
        .map_while(Result::ok)
        .filter(|line| line.contains(pattern))
    {
        lines.push(line)
    }

    Ok(lines)
}

pub fn all(path: &Path) -> Result<Vec<String>> {
    let content = read_lines(path)?;
    let mut lines = Vec::new();

    for line in content.map_while(Result::ok) {
        lines.push(line)
    }

    Ok(lines)
}

fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file).lines())
}

pub fn print(lines: Vec<String>) -> Result<()> {
    for line in lines {
        println!("{}", line);
    }

    Ok(())
}
