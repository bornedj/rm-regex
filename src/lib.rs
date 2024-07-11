//! Fast recursive deletion of files and directories
use std::{
    fs::{read_dir, DirEntry}, io, path::Path
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

pub fn print_collected_entries(entry: &DirEntry) {
    println!("{:?}", entry);
}

pub fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

pub fn collect_entries(
    starting_dir: &String,
) -> Result<Vec<DirEntry>, std::io::Error> {
    let path = Path::new(starting_dir);
    read_dir(path)?.collect()
}

#[cfg(test)]
mod test {
    use super::*;
    mod collect_entries {
        use std::{
            fs::{create_dir_all, remove_dir, remove_file, DirBuilder, File},
            io::Write,
        };

        use super::collect_entries;
        #[test]
        fn should_result_in_err_with_nonexistent_dir() {
            let result = collect_entries(&String::from("nonexistent_dir"));
            assert!(result.is_err());
        }

        #[test]
        fn should_return_ok_if_dir_exists() {
            let path = "./foo";

            // creating empty dir
            DirBuilder::new()
                .create(path)
                .expect("Failed to create dir in test");
            let result = collect_entries(&path.to_owned());

            // remove dir
            remove_dir(path).expect("Failed to delete created dir");

            assert!(result.is_ok());
        }

        #[test]
        fn should_return_all_the_entries_in_a_dir() {
            create_dir_all("./bar/baz").expect("Failed to create dirs");

            let mut f = File::create("./bar/foo.txt").expect("Failed to create new file.");
            f.write_all(b"test")
                .expect("Failed to write to created file.");

            let result = collect_entries(&"./bar/".to_owned());

            // remove mocked files and dir
            remove_dir("./bar/baz").expect("Failed to remove foo/bar");
            remove_file("./bar/foo.txt").expect("Failed to remove foo/baz.txt");
            remove_dir("./bar").expect("Failed to remove foo");

            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 2);
        }
    }
}
