mod fileops;
mod customfiles;
use std::error::Error;
use std::path::PathBuf;
use structopt::StructOpt;
use crate::fileops::FileOperations;
use crate::customfiles::{ScriptFile, MarkupFile, StyleFile, CustomFile}; // Custom file trait required - don't remove

#[derive(StructOpt, Debug)]
struct InputArgs {
    file: PathBuf,
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
