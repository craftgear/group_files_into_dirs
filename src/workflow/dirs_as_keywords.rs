use crate::libs::errors::Error;
use crate::libs::fs::{dirs_in_dir, files_in_dir};
use crate::libs::keywords::extract_keywords;
use crate::libs::stdout::*;

use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;

pub fn execute(pathbuf: PathBuf, verbose: bool) -> Result<Vec<String>, Error> {
    let mut moved_files = vec![];
    let mut update_dirs: HashSet<String> = HashSet::new();

    let dirnames = dirs_in_dir(&pathbuf)?;

    if dirnames.is_empty() {
        return Err(Error::NoKeywordsFound);
    }

    let dir_with_keyword_regexen: Vec<(String, Vec<Regex>)> = dirnames
        .iter()
        .map(|dirname| {
            let keyword_regexen: Vec<Regex> = extract_keywords(dirname)
                .into_iter()
                .filter(|k| k.len() > 1)
                .map(|k| {
                    let re =
                        regex::Regex::new(&format!(r"[\(\[\{{\-_, ]?{k}[\)\]\}}\-_, ]?")).unwrap();
                    re
                })
                .collect();
            (dirname.to_string(), keyword_regexen)
        })
        .collect();

    let filenames = files_in_dir(&pathbuf)?;

    for filename in filenames.iter() {
        for (dirname, keyword_regexen) in dir_with_keyword_regexen.iter() {
            for re in keyword_regexen.iter() {
                if re.is_match(filename) {
                    let src = pathbuf.join(filename);
                    if !src.exists() {
                        already_moved(filename.to_string());
                        continue;
                    }

                    let dst = pathbuf.join(dirname).join(filename);
                    if dst.exists() {
                        already_exists(filename.to_string());
                        continue;
                    }

                    let result = fs::rename(&src, &dst);

                    if result.is_ok() {
                        update_dirs.insert(dirname.to_string());
                        let dst_string = dst.to_str().unwrap().to_string();
                        if verbose {
                            moved(filename.to_string(), dst_string.clone());
                        }
                        moved_files.push(dst_string);
                    } else {
                        error(format!("src {}\ndst {}\n", src.display(), dst.display()));
                        return Err(Error::MoveFileError(result.err().unwrap().to_string()));
                    }
                }
            }
        }
    }

    print_result(update_dirs.len(), Ok(moved_files.clone()));

    Ok(moved_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_dirs_as_keywords_execute() {
        let tmpdir = std::env::temp_dir();
        let tmpdir = tmpdir.join("test_dirs_as_keywords");
        if Path::exists(&tmpdir) {
            fs::remove_dir_all(&tmpdir).unwrap();
        }
        fs::create_dir(&tmpdir).unwrap();

        fs::create_dir(tmpdir.join("inquiry")).unwrap();
        fs::create_dir(tmpdir.join("invoice")).unwrap();
        File::create(tmpdir.join("inquiry_2021-01-01.txt")).unwrap();
        File::create(tmpdir.join("invoice_2021-01-01.txt")).unwrap();
        File::create(tmpdir.join("questionnaire_2021-01-01.txt")).unwrap();

        let result = execute(tmpdir.clone(), false);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            result
                .iter()
                .map(|x| {
                    let p = Path::new(x);
                    let f = p.file_name().unwrap();
                    return f.to_str().unwrap();
                })
                .collect::<Vec<_>>(),
            vec!["inquiry_2021-01-01.txt", "invoice_2021-01-01.txt"]
        );
        assert_eq!(tmpdir.join("inquiry_2021-01-01.txt").exists(), false);
        assert_eq!(tmpdir.join("invoice_2021-01-01.txt").exists(), false);
        assert!(tmpdir.join("questionnaire_2021-01-01.txt").exists());
    }
}
