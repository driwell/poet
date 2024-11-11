use anyhow::Result;
use std::{
    fs::File,
    io::{prelude::*, BufReader, Lines},
    path::Path,
};

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

pub fn all(path: &Path) -> Result<Vec<String>> {
    let content = read_lines(path)?;
    let mut lines = Vec::new();

    for line in content.map_while(Result::ok) {
        lines.push(line)
    }

    Ok(lines)
}

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

pub fn replace(old: &str, new: &str, path: &Path) -> Result<Vec<String>> {
    let content = read_lines(path)?;
    let mut lines = Vec::new();

    for line in content
        .map_while(Result::ok)
        .filter(|line| line.contains(old))
    {
        lines.push(line.replace(old, new));
    }

    Ok(lines)
}

pub fn unfold(lines: Vec<String>, pattern: &str, values: Vec<&str>) -> Result<Vec<String>> {
    let mut unfolded = Vec::new();

    for line in lines {
        for value in &values {
            unfolded.push(line.replace(pattern, value));
        }
    }

    Ok(unfolded)
}
