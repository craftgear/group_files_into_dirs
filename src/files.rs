use owo_colors::OwoColorize;
use std::fs;
use std::fs::rename;
use std::path::Path;

use crate::errors::Error;

pub fn mkdir_for_keywords(keywords: &Vec<String>, basepath: &String) -> Result<(), Error> {
    let basepath = Path::new(basepath);
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

pub fn files_in_dir(path: &String) -> Result<Vec<String>, Error> {
    let files = fs::read_dir(path)?
        .map(|entry| entry.map(|e| e.path()))
        .filter_map(|result| match result {
            Ok(path) => Some(path),
            Err(_) => None,
        })
        .filter(|e| e.is_file())
        .map(|e| e.to_str().unwrap().to_string())
        .collect();

    Ok(files)
}

pub fn move_files_to_dir(files: &Vec<String>, keywords: &Vec<String>) -> Result<i32, Error> {
    let mut success_count = 0;

    for file in files {
        let lower_filename = file.to_lowercase();
        for keyword in keywords {
            let lower_keyword = keyword.to_lowercase();
            if lower_filename.contains(&lower_keyword) {
                let src = Path::new(file);
                let dir = src
                    .parent()
                    .ok_or(Error::ParseError("parent path parse error".to_string()))
                    .unwrap();
                let filename = src
                    .file_name()
                    .ok_or(Error::ParseError("filename parse error".to_string()))
                    .unwrap();
                let dst = Path::new(dir).join(keyword).join(filename);
                let result = rename(src, &dst);
                if result.is_ok() {
                    success_count += 1;
                    let msg = format!("move {} to directory {:?}", file, &dst);
                    println!("{}", msg.green());
                } else {
                    let msg = format!(
                        "src {}, dir {}, filename {:?}, dst {}",
                        src.display(),
                        dir.display(),
                        filename,
                        dst.display()
                    );
                    println!("{}", msg.red());
                    return Err(Error::MoveFileError(result.err().unwrap().to_string()));
                }
            }
        }
    }

    Ok(success_count)
}
