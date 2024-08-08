use clap::Parser;
use owo_colors::OwoColorize;
use std::path::Path;

mod errors;
mod fs;
mod parse_args;

use errors::Error;
use fs::*;
use parse_args::parse_args;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, short)]
    pub keywords: String,
    pub path: String,
}

fn main() -> Result<(), Error> {
    let Args { keywords, path } = Args::parse();
    let (keywords, path) = parse_args(keywords, path)?;
    let pathbuf = Path::new(&path).to_path_buf();
    mkdir_for_keywords(&keywords, &pathbuf)?;
    let files = files_in_dir(&pathbuf)?;
    match move_files_to_dir(&files, &keywords) {
        Ok(result) => {
            let msg = format!(
                "moved {} files to {} directories",
                result.len(),
                keywords.len()
            );
            println!("{}", msg.bold().green());
        }
        Err(err) => {
            let msg = format!("{}", err);
            println!("{}", msg.bold().red());
        }
    }

    Ok(())
}