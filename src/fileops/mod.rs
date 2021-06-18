use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

pub struct FileOperations;

impl FileOperations {
  pub fn write_file(
    path: &PathBuf,
    extention: &str,
    content: &String,
  ) -> Result<(), Box<dyn Error>> {
    let file_name = String::from(path.display().to_string()).replace(".svelte", extention);
    let mut file = File::create(file_name)?;
    file.write_all(content.as_bytes())?;
    Ok(())
  }
  pub fn read_to_string(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut content = String::new();
    for line in reader.lines().map(|l| l.unwrap()) {
      content.push_str(&line);
      content.push_str("\n");
    }
    Ok(content)
  }
}
