mod fileops;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use fileops::FileOperations;

#[derive(StructOpt, Debug)]
struct InputArgs {
    file: PathBuf,
}


trait CustomFile {
    fn new(file_content: &String, file_path: &PathBuf) -> Self;
    fn write(&self) -> Result<(), Box<dyn Error>>;
}

struct ScriptFile {
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
    fn write(&self) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        FileOperations::write_file(&self.svelte_file, ".js", &self.content)
    }
}

struct StyleFile {
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
    fn write(&self) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        FileOperations::write_file(&self.svelte_file, ".css", &self.content)
    }
}

struct MarkupFile {
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
    fn write(&self) -> std::result::Result<(), std::boxed::Box<(dyn std::error::Error + 'static)>> {
        FileOperations::write_file(&self.svelte_file, ".html", &self.content)
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let args = InputArgs::from_args();
    let svelte_file = FileOperations::read_to_string(&args.file)?;
    let script_file = ScriptFile::new(&svelte_file, &args.file);
    let style_file = StyleFile::new(&svelte_file, &args.file);
    let markup_file = MarkupFile::new(&svelte_file, &args.file);
    script_file.write()?;
    style_file.write()?;
    markup_file.write()?;

    Ok(())
}
