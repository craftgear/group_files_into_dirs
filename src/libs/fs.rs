use owo_colors::OwoColorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::libs::errors::Error;

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
                    let msg = format!("{} is already moved", filename);
                    println!("{}", msg.yellow());
                    continue;
                }
                // create a new directory for the keyword.
                let dirname = mkdir_for_keyword(keyword.to_string(), basepath)?;

                let dst = &basepath.join(dirname).join(filename);
                // destination file is already exists.
                if dst.exists() {
                    let msg = format!("{} is already exists in {}", filename, keyword);
                    println!("{}", msg.yellow());
                    continue;
                }
                let result = fs::rename(src, &dst);
                if result.is_ok() {
                    moved_files.push(dst.to_str().unwrap().to_string());
                    if verbose {
                        let msg = format!("move {} to directory {}", filename, &dst.display());
                        println!("{}", msg.blue());
                    }
                } else {
                    let msg = format!(
                        "src {}\nfilename {}\ndst {}\n",
                        src.display(),
                        filename,
                        dst.display()
                    );
                    println!("{}", msg.red());
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
    match move_files_to_dir(&pathbuf, &files, &keywords, verbose) {
        Ok(result) => {
            if result.len() == 0 {
                let msg = format!("no files are moved.");
                println!("{}", msg.bold().blue());
            } else {
                let msg = format!(
                    "moved {} files to {} directories.",
                    result.len(),
                    keywords.len()
                );
                println!("{}", msg.bold().green());
            }
        }
        Err(err) => {
            let msg = format!("{}", err);
            println!("Error: {}.", msg.bold().red());
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

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
        for file in FILES.iter() {
            let path = tmpdir.join(file);
            fs::File::create(&path).unwrap();
        }

        let result = files_in_dir(&tmpdir).unwrap();
        assert_eq!(result.len(), FILES.len());
        for file in FILES.iter() {
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
        for file in FILES.iter() {
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
}
