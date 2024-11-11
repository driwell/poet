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

pub fn read(path: &Path) -> Result<Vec<String>> {
    let content = read_lines(path)?;
    let mut lines = Vec::new();

    for line in content.map_while(Result::ok) {
        lines.push(line)
    }

    Ok(lines)
}

pub fn find(pattern: &str, lines: Vec<String>) -> Result<Vec<String>> {
    let mut found = Vec::new();

    for line in lines {
        if line.contains(pattern) {
            found.push(line)
        }
    }

    Ok(found)
}

pub fn replace(old: &str, new: &str, lines: Vec<String>) -> Result<Vec<String>> {
    let mut replaced = Vec::new();

    for line in lines {
        if line.contains(old) {
            replaced.push(line.replace(old, new));
        }
    }

    Ok(replaced)
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
