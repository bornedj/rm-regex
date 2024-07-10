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

#[cfg(test)]
mod test {
    use super::*;
    mod collect_entries {
        use std::fs::{remove_dir, DirBuilder};

        use super::collect_entries;
        #[test]
        fn should_result_in_err_with_nonexistent_dir() {
            let result = collect_entries(&String::from("nonexistent_dir"));
            assert!(result.is_err());
        }

        #[test]
        fn should_return_ok_if_dir_exists() {
            let path = "foo";

            // creating empty dir
            DirBuilder::new()
                .create(path).expect("Failed to create dir in test");
            let result = collect_entries(&path.to_owned());

            // remove dir
            remove_dir(path).expect("Failed to delete created dir");

            assert!(result.is_ok());
        }
    }

}
