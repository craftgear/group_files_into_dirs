use spinners::{Spinner, Spinners};
use std::path::PathBuf;

use crate::libs::errors::Error;
use crate::libs::extract_keywords::*;
use crate::libs::fs::files_in_dir;
use crate::libs::tui::run;

pub fn execute(pathbuf: &PathBuf) -> Result<Vec<String>, Error> {
    println!("");
    let mut sp = Spinner::new(
        Spinners::CircleHalves,
        "Extracting keywords with 3 or more charaters from filenames".into(),
    );

    let filenames = files_in_dir(&pathbuf)?;
    let keyword_hash = extract_keywords_and_count_from_filenames(&filenames);
    let keyword_vec = sort_by_count_and_keyword_length(keyword_hash);

    // filter keywords that appear more than once.
    let keywords = keyword_vec
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .collect::<Vec<_>>();

    sp.stop_with_newline();

    let selected_keywords = run(keywords)?;

    Ok(selected_keywords)
}
