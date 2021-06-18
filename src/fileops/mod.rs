use anyhow::{Context, Result};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct FileOperations;

pub fn path_to_string(path: &Path) -> String {
  String::from(path.display().to_string())
}

impl FileOperations {
  pub fn write_file(path: &PathBuf, extention: &str, content: &String) -> Result<()> {
    let file_name = path_to_string(&path).replace(".svelte", extention);
    let file_name = file_name.as_str();
    let mut file =
      File::create(file_name).with_context(|| format!("Unable to create file `{}`", file_name))?;

    file
      .write_all(content.as_bytes())
      .with_context(|| format!("Unable to write to file `{}`", file_name))?;
    Ok(())
  }
  pub fn read_to_string(path: &PathBuf) -> Result<String> {
    let file = File::open(path).with_context(|| format!("Unable to read file `{}`", path_to_string(&path)))?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines().map(|l| l.unwrap()) {
      content.push_str(&line);
      content.push_str("\n");
    }
    Ok(content)
  }
}
