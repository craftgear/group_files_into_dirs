use spinners::{Spinner, Spinners};
use std::path::PathBuf;

use crate::libs::errors::Error;
use crate::libs::extract_keywords::*;
use crate::libs::fs::files_in_dir;
use crate::libs::tui::run;

pub fn execute(pathbuf: &PathBuf) -> Result<Vec<String>, Error> {
    let mut sp = Spinner::new(
        Spinners::CircleHalves,
        "Extracting keywords from filenames...".into(),
    );

    let filenames = files_in_dir(&pathbuf)?;

    let keyword_hash = extract_keywords_from_filenames(&filenames);
    let mut keyword_vec: Vec<(String, usize)> = keyword_hash.into_iter().collect();
    keyword_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // filter keywords that appear more than once.
    let keywords = keyword_vec
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .collect::<Vec<_>>();

    sp.stop();

    let selected_keywords = run(keywords)?;

    Ok(selected_keywords)
}
