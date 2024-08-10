use clap::Parser;
use std::path::Path;

mod libs;

use libs::errors::Error;
use libs::fs::move_files_to_dir_by_keywords;
use libs::interactive;
use libs::parse_args::parse_args;

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
    #[clap(help = "Verbose output", long, short)]
    pub verbose: bool,
}

fn main() -> Result<(), Error> {
    let Args {
        keywords,
        path,
        verbose,
    } = Args::parse();

    if let Some(keywords) = keywords {
        return use_keywords(keywords, path, verbose);
    }

    interactive_mode(path, verbose)
}

fn use_keywords(keywords: String, path: String, verbose: bool) -> Result<(), Error> {
    let (keywords, path) = parse_args(keywords, path)?;
    let pathbuf = Path::new(&path).to_path_buf();

    move_files_to_dir_by_keywords(keywords, pathbuf, verbose)
}

fn interactive_mode(path: String, verbose: bool) -> Result<(), Error> {
    let pathbuf = Path::new(&path).to_path_buf();

    let keywords = interactive::execute(&pathbuf)?;

    move_files_to_dir_by_keywords(keywords, pathbuf, verbose)
}
