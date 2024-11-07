use anyhow::Result;
use std::{fs, path::Path};

pub fn find(pattern: &str, path: &Path) -> Result<()> {
    let content = fs::read_to_string(path)?;

    for line in content.lines() {
        if line.contains(pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
