use clap::Parser;
use owo_colors::OwoColorize;
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
    #[clap(help = "Use directory as keyword", long, short)]
    pub dir_as_keyword: bool,
}

fn main() -> Result<(), Error> {
    let Args {
        keywords,
        path,
        verbose,
        dir_as_keyword,
    } = Args::parse();

    if !Path::new(&path).exists() {
        let msg = format!("Error: path {} does not exist", path);
        println!("{}", msg.red());
        return Ok(());
    }

    if let Some(keywords) = keywords {
        return use_keywords(keywords, path, verbose);
    }

    if dir_as_keyword {
        return use_dirs_as_keywords(path, verbose);
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

fn use_dirs_as_keywords(path: String, verbose: bool) -> Result<(), Error> {
    Ok(())
}
