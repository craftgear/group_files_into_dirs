use owo_colors::OwoColorize;
use std::fs;
use std::fs::rename;
use std::path::{Path, PathBuf};

use crate::libs::errors::Error;

pub fn mkdir_for_keywords(keywords: &Vec<String>, basepath: &PathBuf) -> Result<(), Error> {
    if !Path::exists(&basepath) {
        return Err(Error::IOError(format!(
            "path {} is not exists",
            basepath.display()
        )));
    }

    for keyword in keywords {
        let full_path_dirname = Path::new(basepath).join(keyword);
        if Path::exists(&full_path_dirname) {
            continue;
        }
        fs::create_dir(full_path_dirname)?;
    }

    Ok(())
}

pub fn files_in_dir(path: &PathBuf) -> Result<Vec<String>, Error> {
    let files = fs::read_dir(path)?
        .filter_map(|e| {
            if let Ok(entry) = e {
                let filename = entry.file_name().into_string().ok()?;

                if filename.starts_with(".") {
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
                if src.exists() == false {
                    let msg = format!("{} is already moved", filename);
                    println!("{}", msg.yellow());
                    continue;
                }
                let dst = &basepath.join(keyword).join(filename);
                let result = rename(src, &dst);
                if result.is_ok() {
                    moved_files.push(dst.to_str().unwrap().to_string());
                    let msg = format!("move {} to directory {:?}", filename, &dst);
                    println!("{}", msg.green());
                } else {
                    let msg = format!(
                        "src {}, filename {:?}, dst {}",
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mkdir_for_keywords() {
        let tmpdir = std::env::temp_dir();
        mkdir_for_keywords(&vec![String::from("foo"), String::from("bar")], &tmpdir).unwrap();
        let foo_dir = tmpdir.join("foo");
        let bar_dir = tmpdir.join("bar");
        assert_eq!(foo_dir.exists(), true);
        assert_eq!(bar_dir.exists(), true);
        fs::remove_dir_all(&foo_dir).unwrap();
        fs::remove_dir_all(&bar_dir).unwrap();
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
        let _ = mkdir_for_keywords(&keywords, &tmpdir);
        let files = files_in_dir(&tmpdir).unwrap();
        let moved_files = move_files_to_dir(&tmpdir, &files, &keywords).unwrap();
        assert_eq!(moved_files.len(), 6);

        for file in moved_files.iter() {
            assert_eq!(Path::new(file).exists(), true);
        }
    }
}
