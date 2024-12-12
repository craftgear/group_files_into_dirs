use clap::Parser;
use std::path::PathBuf;

mod libs;
mod workflow;

use libs::errors::Error;
use libs::fs::{move_files_to_dir_by_keywords, parse_path};
use libs::parse_args::parse_args;
use libs::tui;
use workflow::{dirs_as_keywords, interactive};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(
        help = "Specify keywords for grouping files, cannot be used with interactive mode",
        long,
        short
    )]
    pub keywords: Option<String>,
    #[clap(required = true)]
    pub path: String,
    #[clap(
        default_value_t = false,
        help = "Verbose output (default: false)",
        long,
        short
    )]
    pub verbose: bool,
    #[clap(
        default_value_t = false,
        help = "Use directory names as keyword (default: false)",
        long,
        short
    )]
    pub dir_as_keyword: bool,
    #[clap(
        default_value_t = false,
        help = "interactive mode: use spaces as delimiters (default: false)",
        long,
        short
    )]
    pub space_delimiter: bool,
}

fn main() -> Result<(), Error> {
    let Args {
        keywords,
        path,
        verbose,
        dir_as_keyword,
        space_delimiter,
    } = Args::parse();

    println!("space_delimiter is {:?}", space_delimiter);
    let pathbuf = parse_path(path)?;

    if let Some(keywords) = keywords {
        return use_keywords(keywords, pathbuf, verbose);
    }

    if dir_as_keyword {
        return use_dirs_as_keywords(pathbuf, verbose);
    }

    interactive_mode(pathbuf, verbose, space_delimiter)
}

fn use_keywords(keywords: String, pathbuf: PathBuf, verbose: bool) -> Result<(), Error> {
    let keywords = parse_args(keywords)?;

    move_files_to_dir_by_keywords(keywords, pathbuf, verbose)
}

fn interactive_mode(pathbuf: PathBuf, verbose: bool, space_delimiter: bool) -> Result<(), Error> {
    let keywords = interactive::execute(&pathbuf, tui::run, space_delimiter)?;

    move_files_to_dir_by_keywords(keywords, pathbuf, verbose)
}

fn use_dirs_as_keywords(pathbuf: PathBuf, verbose: bool) -> Result<(), Error> {
    let _ = dirs_as_keywords::execute(pathbuf, verbose)?;
    Ok(())
}
