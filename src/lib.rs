//! Fast recursive deletion of files and directories
use clap::{ArgAction, Parser};
use std::{fs::{read_dir, DirEntry}, path::Path};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    expression: String,

    #[arg(short = 'r', long, value_name = "ROOT")]
    root_dir: Option<String>,

    #[arg(short = 'd', long, action=ArgAction::SetTrue)]
    dir: bool,

    #[arg(short = 'f', long, action=ArgAction::SetTrue)]
    file: bool,
}

type ParsedArgs = (String, String, bool, bool);

pub fn destructure_args() -> ParsedArgs {
    let cli = Cli::parse();
    match cli.root_dir {
        Some(path) => (cli.expression, path, cli.dir, cli.file),
        None => (cli.expression, "./".to_owned(), cli.dir, cli.file)
    }
}

pub fn collect_entries(starting_dir: &String) -> Result<Vec<DirEntry>, std::io::Error> {
    let path = Path::new(starting_dir);

    let read_result = read_dir(path)?;
    read_result.collect()
}
}
