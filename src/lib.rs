use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn find(pattern: &str, path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}

pub fn print(path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    for line in content.lines() {
        println!("{}", line);
    }

    Ok(())
}
