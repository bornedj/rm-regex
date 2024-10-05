//! Fast recursive deletion of files and directories
use std::{
    fs::{read_dir, DirEntry}, path::PathBuf
};

use clap::Parser;
use regex::Regex;


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    expression: String,

    #[arg(short = 'r', long, value_name = "ROOT")]
    root_dir: Option<String>,

    // #[arg(short = 'd', long, action=ArgAction::SetTrue)]
    // dir: bool,
    //
    // #[arg(short = 'f', long, action=ArgAction::SetTrue)]
    // file: bool,
}

type ParsedArgs = (String, String);

pub fn destructure_args() -> ParsedArgs {
    let cli = Cli::parse();

    match cli.root_dir {
        Some(path) => (cli.expression, path),
        None => (cli.expression, "./".to_owned()),
    }
}

pub fn visit_dirs(dir: &PathBuf, expression: &Regex) -> Result<(), std::io::Error> {
    let entries: Vec<DirEntry> = read_dir(dir)?
        .filter_map(|result| result.ok())
        .collect();

    for entry in entries {
        let path = entry.path();
        let name: String = path.file_name().unwrap().to_string_lossy().into();
        if expression.is_match(&name) {
            delete(path);
            continue;
        }
        if path.is_dir() {
            visit_dirs(&path, expression)?
        }
        // match path {
        //     path if path.is_dir() => {
        //         delete();
        //     },
        //     path if path.is_file() => delete(),
        //     path if path.is_symlink() => delete(),
        //     _ => unreachable!(),
        // };
    };

    // let name: String = dir
    //     .file_name()
    //     .unwrap_or(OsStr::new("."))
    //     .to_string_lossy()
    //     .into();

    Ok(())
}

fn delete(path: PathBuf) {
    if path.is_dir() {
        println!("Deleting directory: {:?}",path);
    }

    if path.is_file() {
        println!("Deleting file: {:?}",path);
    }
}
