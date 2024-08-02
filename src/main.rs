// DONE: 引数で分類キーワードを受け取る
// DONE: 分類キーワードのディレクトリがなければ作成する
// DONE: 指定したディレクトリのファイルを読み込む
// DONE: ファイル名が分類キーワードを含む場合、そのファイルを指定したディレクトリに移動する
// TODO: テストを書く

use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;
use std::fs::rename;
use std::path::Path;

#[derive(Clone, Debug, thiserror::Error)]
enum Error {
    #[error("move file error: {0}")]
    MoveFileError(String),
    #[error("keyword length error: {0}")]
    KeywordLengthError(String),
    #[error("io error: {0}")]
    IOError(String),
    // #[error("parse error: {0}")]
    // ParseError(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IOError(e.to_string())
    }
}

fn main() -> Result<(), Error> {
    let (keywords, path) = parse_args()?;
    mkdir_for_keywords(&keywords, &path)?;
    let files = files_in_dir(&path)?;
    match move_files_to_dir(&files, &keywords) {
        Ok(success_count) => {
            let msg = format!(
                "moved {} files to {} directories",
                success_count,
                keywords.len()
            );
            println!("{}", msg.green());
        }
        Err(err) => {
            let msg = format!("failed to move files {}", err);
            println!("{}", msg.red());
        }
    }

    Ok(())
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, short)]
    keywords: String,
    path: String,
}

fn parse_args() -> Result<(Vec<String>, String), Error> {
    let Args { keywords, path } = Args::parse();

    let keywords = keywords
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    for keyword in &keywords {
        if keyword.len() < 2 {
            return Err(Error::KeywordLengthError(
                "keyword length must be more than 2".to_string(),
            ));
        }
    }

    if cfg!(windows) {
        if path.ends_with("\"") {
            let win_path = path.trim_end_matches("\"").to_string();
            return Ok((keywords, win_path));
        }
    }

    Ok((keywords, path))
}

fn mkdir_for_keywords(keywords: &Vec<String>, basepath: &String) -> Result<(), Error> {
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

fn files_in_dir(path: &String) -> Result<Vec<String>, Error> {
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

fn move_files_to_dir(files: &Vec<String>, keywords: &Vec<String>) -> Result<i32, Error> {
    let mut success_count = 0;
    for file in files {
        let lower_file = file.to_lowercase();
        for keyword in keywords {
            let lower_keyword = keyword.to_lowercase();
            if lower_file.contains(&lower_keyword) {
                let src = Path::new(file);
                // TODO: map_errで parse errorを返すようにする
                let dir = src.parent().unwrap();
                let filename = src.file_name().unwrap();
                let dst = Path::new(dir).join(keyword).join(filename);
                let result = rename(src, &dst);
                if result.is_ok() {
                    success_count += 1;
                    println!("move {} to directory {:?}", file, &dst);
                } else {
                    return Err(Error::MoveFileError(result.err().unwrap().to_string()));
                }
            }
        }
    }

    Ok(success_count)
}
