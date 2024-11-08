use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Lines},
    path::Path,
};

pub fn find(pattern: &str, path: &Path) -> Result<Vec<String>> {
    let lines = read_lines(path)?;
    let mut correct_lines = Vec::new();

    for line in lines
        .map_while(Result::ok)
        .filter(|line| line.contains(pattern))
    {
        correct_lines.push(line)
    }

    Ok(correct_lines)
}

pub fn all(path: &Path) -> Result<Vec<String>> {
    let lines = read_lines(path)?;
    let mut correct_lines = Vec::new();

    for line in lines.map_while(Result::ok) {
        correct_lines.push(line)
    }

    Ok(correct_lines)
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
