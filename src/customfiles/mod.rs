use crate::fileops::FileOperations;
// use std::error::Error;
use anyhow::{Context, Result};
use std::path::PathBuf;

pub trait CustomFile {
  fn new(file_content: &String, file_path: &PathBuf) -> Self;
  fn write(&self) -> Result<()>;
  fn parse(content: &str, start_tag: &str, end_tag: &str) -> Result<String> {
    let mut parsed_content = String::new();
    let mut is_parsing_tag_content = false;
    for line in content.lines() {
      if line.starts_with(start_tag) {
        is_parsing_tag_content = true;
      } else if line.starts_with(end_tag) {
        is_parsing_tag_content = false;
      } else if is_parsing_tag_content {
        parsed_content.push_str(&line);
        parsed_content.push_str("\n");
      }
    }
    Ok(parsed_content)
  }
}

pub struct ScriptFile {
  content: String,
  svelte_file: PathBuf,
}

impl CustomFile for ScriptFile {
  fn new(svelte_file_content: &String, svelte_file_path: &PathBuf) -> ScriptFile {
    ScriptFile {
      content: svelte_file_content.to_string(),
      svelte_file: svelte_file_path.clone(),
    }
  }
  fn write(&self) -> Result<()> {
    FileOperations::write_file(
      &self.svelte_file,
      ".js",
      &Self::parse(&self.content, "<script", "</script")
        .with_context(|| "error parsing script content")?,
    )
  }
}

pub struct StyleFile {
  content: String,
  svelte_file: PathBuf,
}

impl CustomFile for StyleFile {
  fn new(svelte_file_content: &String, svelte_file_path: &PathBuf) -> StyleFile {
    StyleFile {
      content: svelte_file_content.to_string(),
      svelte_file: svelte_file_path.clone(),
    }
  }
  fn write(&self) -> Result<()> {
    FileOperations::write_file(
      &self.svelte_file,
      ".css",
      &Self::parse(&self.content, "<style", "</style")
        .with_context(|| "error parsing style content")?,
    )
  }
}

pub struct MarkupFile {
  content: String,
  svelte_file: PathBuf,
}

impl CustomFile for MarkupFile {
  fn new(svelte_file_content: &String, svelte_file_path: &PathBuf) -> MarkupFile {
    MarkupFile {
      content: svelte_file_content.to_string(),
      svelte_file: svelte_file_path.clone(),
    }
  }
  fn write(&self) -> Result<()> {
    FileOperations::write_file(
      &self.svelte_file,
      ".html",
      &Self::parse(&self.content, "", "").with_context(|| "error parsing markup content")?,
    )
  }
  fn parse(content: &str, start_tag: &str, end_tag: &str) -> Result<String> {
    let mut parsed_content = String::new();
    let mut is_parsing_tag_content = false;
    let is_inside_tags = start_tag != "";
    if is_inside_tags {
      for line in content.lines() {
        if line.starts_with(start_tag) {
          is_parsing_tag_content = true;
        } else if line.starts_with(end_tag) {
          is_parsing_tag_content = false;
        } else if is_parsing_tag_content {
          parsed_content.push_str(&line);
          parsed_content.push_str("\n");
        }
      }
    } else {
      is_parsing_tag_content = true;
      for line in content.lines() {
        if line.starts_with("<script") || line.starts_with("<style") {
          is_parsing_tag_content = false;
        } else if line.starts_with("</script") || line.starts_with("</style") {
          is_parsing_tag_content = true;
        } else if is_parsing_tag_content {
          parsed_content.push_str(&line);
          parsed_content.push_str("\n");
        }
      }
    }
    Ok(parsed_content)
  }
}
