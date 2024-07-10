//! Fast recursive deletion of files and directories
use clap::{ArgAction, Parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    expression: String,

    #[arg(short, long, value_name = "ROOT")]
    starting_dir: Option<PathBuf>,

    #[arg(short = 'd', long, action=ArgAction::SetTrue)]
    dir: bool,

    #[arg(short = 'f', long, action=ArgAction::SetTrue)]
    file: bool,
}

/// Represents the parsed arguments in the order they are passed
type ParsedArgs = (String, Option<PathBuf>, bool, bool);

/// Returns the parsed arguments
pub fn destructure_args() -> ParsedArgs {
    let cli = Cli::parse();
    (cli.expression, cli.starting_dir, cli.dir, cli.file)
}
