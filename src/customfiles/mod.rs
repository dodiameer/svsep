use crate::fileops::FileOperations;
// use std::error::Error;
use std::path::PathBuf;
use anyhow::Result;

pub trait CustomFile {
  fn new(file_content: &String, file_path: &PathBuf) -> Self;
  fn write(&self) -> Result<()>;
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
   fn write(
    &self,
  ) -> Result<()> {
    FileOperations::write_file(&self.svelte_file, ".js", &self.content)
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
   fn write(
    &self,
  ) -> Result<()> {
    FileOperations::write_file(&self.svelte_file, ".css", &self.content)
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
   fn write(
    &self,
  ) -> Result<()> {
    FileOperations::write_file(&self.svelte_file, ".html", &self.content)
  }
}
