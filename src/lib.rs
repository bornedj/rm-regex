//! Fast recursive deletion of files and directories
use std::{
    fs::{metadata, read_dir, read_link, DirEntry, Metadata}, io, path::PathBuf,
    ffi::OsStr
};

use clap::{ArgAction, Parser};


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
        None => (cli.expression, "./".to_owned(), cli.dir, cli.file),
    }
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub struct Symlink {
    pub name: String,
    pub target: String,
    pub metadata: Metadata,
}

#[derive(Debug)]
pub struct Directory {
    pub name: String,
    pub entries: Vec<FileTree>
}

#[derive(Debug)]
pub enum FileTree {
    DirNode(Directory),
    FileNode(File),
    LinkNode(Symlink),
}

pub fn visit_dirs(dir: &PathBuf) -> io::Result<Directory> {
    let entries: Vec<DirEntry> = read_dir(dir)?
        .filter_map(|result| result.ok())
        .collect();

    let mut directory: Vec<FileTree> = Vec::with_capacity(entries.len());

    for entry in entries {
        let path = entry.path();
        let name: String = path.file_name().unwrap().to_string_lossy().into();
        let metadata = metadata(&path).unwrap();
        let node = match path {
            path if path.is_dir() => {
                FileTree::DirNode(
                    visit_dirs(dir)?
                    )
            },
            path if path.is_file() => FileTree::FileNode(File {
                name: name.into(),
                metadata,
            }),
            path if path.is_symlink() => FileTree::LinkNode(Symlink {
                name: name.into(),
                target: read_link(path).unwrap().to_string_lossy().into(),
                metadata,
            }),
            _ => unreachable!(),
        };
        directory.push(node);
    };

    let name: String = dir
        .file_name()
        .unwrap_or(OsStr::new("."))
        .to_string_lossy()
        .into();

    Ok(Directory {
        name,
        entries: directory
    })
}
