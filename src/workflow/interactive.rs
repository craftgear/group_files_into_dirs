use spinners::{Spinner, Spinners};
use std::path::PathBuf;

use crate::libs::errors::Error;
use crate::libs::fs::files_in_dir;
use crate::libs::keywords::*;

pub fn execute(
    pathbuf: &PathBuf,
    run: fn(Vec<(String, usize)>) -> Result<Vec<String>, Error>,
) -> Result<Vec<String>, Error> {
    println!("");
    let mut sp = Spinner::new(
        Spinners::CircleHalves,
        "Extracting keywords with 2 or more charaters from filenames".into(),
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

    if keywords.is_empty() {
        return Err(Error::NoKeywordsFound);
    }

    let selected_keywords = run(keywords)?;

    Ok(selected_keywords)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::path::Path;

    #[test]
    fn test_interactive_execute() {
        let tmpdir = std::env::temp_dir();
        let tmpdir = tmpdir.join("test_interactive");
        if Path::exists(&tmpdir) {
            fs::remove_dir_all(&tmpdir).unwrap();
        }
        fs::create_dir(&tmpdir).unwrap();

        File::create(tmpdir.join("inquiry_2021-01-01.txt")).unwrap();
        File::create(tmpdir.join("inquiry_2022-01-01.txt")).unwrap();
        File::create(tmpdir.join("invoice_2021-02-01.txt")).unwrap();
        File::create(tmpdir.join("invoice_2022-02-01.txt")).unwrap();
        File::create(tmpdir.join("questionnaire_2021-03-01.txt")).unwrap();

        let tui_mock = |keywords: Vec<(String, usize)>| -> Result<Vec<String>, Error> {
            Ok(keywords
                .iter()
                .map(|(keyword, _)| keyword.clone())
                .collect())
        };

        let expect = vec!["01", "2021", "inquiry", "invoice", "2022", "02"];
        let result = execute(&tmpdir, tui_mock).unwrap();
        assert_eq!(
            result
                .iter()
                .all(|keyword| expect.contains(&keyword.as_str())),
            true
        );
    }
}
