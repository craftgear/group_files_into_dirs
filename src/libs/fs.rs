use std::fs;
use std::path::{Path, PathBuf};

use crate::libs::errors::Error;
use crate::libs::stdout::*;

pub fn parse_path(mut path: String) -> Result<PathBuf, Error> {
    if cfg!(windows) {
        if path.ends_with("\"") {
            path = path.trim_end_matches("\"").to_string();
        }
    }

    if !Path::new(&path).exists() {
        return Err(Error::IOError(format!("path {} does not exist", path)));
    }

    return Ok(Path::new(&path).to_path_buf());
}

pub fn mkdir_for_keyword(keyword: String, basepath: &PathBuf) -> Result<String, Error> {
    if !Path::exists(&basepath) {
        return Err(Error::IOError(format!(
            "path {} is not exists",
            basepath.display()
        )));
    }

    let full_path_dirname = Path::new(basepath).join(&keyword);
    if Path::exists(&full_path_dirname) {
        return Ok(keyword);
    }
    fs::create_dir(full_path_dirname)?;

    Ok(keyword)
}

pub fn files_in_dir(path: &PathBuf) -> Result<Vec<String>, Error> {
    let files = fs::read_dir(path)?
        .filter_map(|e| {
            if let Ok(entry) = e {
                let metadata = entry.metadata().ok()?;
                let filename = entry.file_name().into_string().ok()?;

                if filename.starts_with(".") || metadata.is_dir() {
                    return None;
                }

                return Some(filename);
            }
            None
        })
        .collect();

    Ok(files)
}

pub fn move_files_to_dir(
    basepath: &PathBuf,
    filenames: &Vec<String>,
    keywords: &Vec<String>,
    verbose: bool,
) -> Result<Vec<String>, Error> {
    let mut moved_files = vec![];

    for filename in filenames {
        let lower_filename = filename.to_lowercase();
        for keyword in keywords {
            // if filename is the same as keyword, it is a directory so skip it.
            if filename == keyword {
                continue;
            }
            let lower_keyword = keyword.to_lowercase();
            if lower_filename.contains(&lower_keyword) {
                let src = &basepath.join(filename);
                // files could be moved by other keywords.
                if !src.exists() {
                    already_moved(filename.to_string());
                    continue;
                }
                // create a new directory for the keyword.
                let dirname = mkdir_for_keyword(keyword.to_string(), basepath)?;

                let dst = &basepath.join(dirname).join(filename);
                // destination file is already exists.
                if dst.exists() {
                    already_exists(filename.to_string());
                    continue;
                }
                let result = fs::rename(src, &dst);
                if result.is_ok() {
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

    Ok(moved_files)
}

pub fn move_files_to_dir_by_keywords(
    keywords: Vec<String>,
    pathbuf: PathBuf,
    verbose: bool,
) -> Result<(), Error> {
    let files = files_in_dir(&pathbuf)?;

    let result = move_files_to_dir(&pathbuf, &files, &keywords, verbose);
    print_result(keywords.len(), result);

    Ok(())
}

pub fn dirs_in_dir(path: &PathBuf) -> Result<Vec<String>, Error> {
    let dirs = fs::read_dir(path)?
        .filter_map(|e| {
            if let Ok(entry) = e {
                let metadata = entry.metadata().ok()?;
                let filename = entry.file_name().into_string().ok()?;

                if filename.starts_with(".") || !metadata.is_dir() {
                    return None;
                }

                return Some(filename);
            }
            None
        })
        .collect();

    Ok(dirs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_check_path_return_error_unless_path_exists() {
        if let Err(e) = parse_path("hogehoge".to_string()) {
            assert_eq!(e.to_string(), "io error: path hogehoge does not exist");
        };
    }

    #[test]
    fn test_check_path_can_parse_windows_terminal_path() {
        if let Ok(pathbuf) = parse_path("path\"".to_string()) {
            assert_eq!(pathbuf, PathBuf::from("path"));
        };
    }

    #[test]
    fn test_mkdir_for_keywords() {
        let tmpdir = std::env::temp_dir();
        for keyword in vec!["foo", "bar", "(baz)", "[aaa]", "{bbb}"] {
            let dirname = mkdir_for_keyword(keyword.to_string(), &tmpdir).unwrap();
            let new_dir = tmpdir.join(dirname);
            assert_eq!(new_dir.exists(), true);
        }
    }

    const FILES: [&str; 7] = [
        "inquiry_1.txt",
        "inquiry_2.md",
        "invoice_1.txt",
        "invoice_2.md",
        "invoice_3.pdf",
        "inquiry_invoice.pdf",
        "questionnaire_1.xls",
    ];

    #[test]
    fn test_files_in_dir() {
        let tmpdir = std::env::temp_dir();
        let tmpdir = tmpdir.join("test_files_in_dir");
        if Path::exists(&tmpdir) {
            fs::remove_dir_all(&tmpdir).unwrap();
        }
        fs::create_dir(&tmpdir).unwrap();

        for file in FILES {
            let path = tmpdir.join(file);
            fs::File::create(&path).unwrap();
        }

        let result = files_in_dir(&tmpdir).unwrap();
        assert_eq!(result.len(), FILES.len());
        for file in FILES {
            assert_eq!(result.contains(&file.to_string()), true);
        }
    }

    #[test]
    fn test_move_files_to_dir() {
        let tmpdir = std::env::temp_dir();
        let tmpdir = tmpdir.join("test_move_files_to_dir");
        if Path::exists(&tmpdir) {
            fs::remove_dir_all(&tmpdir).unwrap();
        }
        fs::create_dir(&tmpdir).unwrap();

        for file in FILES {
            let path = tmpdir.join(file);
            fs::File::create(&path).unwrap();
        }

        let keywords = vec![String::from("inquiry"), String::from("invoice")];
        let files = files_in_dir(&tmpdir).unwrap();
        let moved_files = move_files_to_dir(&tmpdir, &files, &keywords, true).unwrap();
        assert_eq!(moved_files.len(), 6);

        for file in moved_files.iter() {
            assert_eq!(Path::new(file).exists(), true);
        }
    }

    #[test]
    fn test_dirs_in_dir() {
        let tmpdir = std::env::temp_dir();
        let tmpdir = tmpdir.join("test_dirs_in_dir");
        if Path::exists(&tmpdir) {
            fs::remove_dir_all(&tmpdir).unwrap();
        }
        fs::create_dir(&tmpdir).unwrap();

        let dirnames = vec!["inquiry", "invoice", "questionnaire"];
        for dir in dirnames.iter() {
            let path = tmpdir.join(dir);
            println!("path is {:?}", path);
            fs::create_dir(&path).unwrap();
        }

        let result = dirs_in_dir(&tmpdir).unwrap();
        assert_eq!(result.len(), dirnames.len());
        let result_set: HashSet<&String> = result.iter().collect();
        assert_eq!(
            dirnames.iter().all(|x| result_set.contains(&x.to_string())),
            true
        );
    }
}
