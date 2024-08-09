use clap::Parser;
use owo_colors::OwoColorize;
use std::path::Path;

mod libs;

use libs::errors::Error;
use libs::fs::*;
use libs::interactive;
use libs::parse_args::parse_args;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(
        help = "keywords for grouping files, cannot be used with interactive mode",
        long,
        short
    )]
    pub keywords: Option<String>,
    #[clap(required = true)]
    pub path: String,
}

fn main() -> Result<(), Error> {
    let Args { keywords, path } = Args::parse();

    if let Some(keywords) = keywords {
        println!("keywords: {:?}", keywords);
        return use_keywords(keywords, path);
    }

    interactive_mode(path)
}

fn use_keywords(keywords: String, path: String) -> Result<(), Error> {
    let (keywords, path) = parse_args(keywords, path)?;
    let pathbuf = Path::new(&path).to_path_buf();
    mkdir_for_keywords(&keywords, &pathbuf)?;
    let files = files_in_dir(&pathbuf)?;
    match move_files_to_dir(&pathbuf, &files, &keywords) {
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

fn interactive_mode(path: String) -> Result<(), Error> {
    let filenames = files_in_dir(&Path::new(&path).to_path_buf())?;

    let keywords = interactive::execute(filenames);
    for (keyword, count) in &keywords {
        if keyword.starts_with("獣") {
            let msg = format!("{:>5}: {}", count, keyword);
            println!("{}", msg);
        }
    }

    Ok(())
}
