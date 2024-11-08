use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use anyhow::{Context, Result};
use std::{fs, path::Path};

pub fn find(pattern: &str, path: &Path) -> Result<()> {
    let content = File::open(path)?;
    let reader = BufReader::new(content);

    // TODO: implement this properly
    for line in reader.lines() {
        let un = line?;
        if un.contains(pattern) {
            println!("{}", un);
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
