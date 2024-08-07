// DONE: 引数で分類キーワードを受け取る
// DONE: 分類キーワードのディレクトリがなければ作成する
// DONE: 指定したディレクトリのファイルを読み込む
// DONE: ファイル名が分類キーワードを含む場合、そのファイルを指定したディレクトリに移動する
// DONE: ファイルを分割する
// TODO: テストを書く
// TODO: READMEを書く

use clap::Parser;
use owo_colors::OwoColorize;

mod errors;
mod files;
mod parse_args;

use errors::Error;
use files::*;
use parse_args::parse_args;

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(long, short)]
    pub keywords: String,
    pub path: String,
}

fn main() -> Result<(), Error> {
    let Args { keywords, path } = Args::parse();
    let (keywords, path) = parse_args(keywords, path)?;
    mkdir_for_keywords(&keywords, &path)?;
    let files = files_in_dir(&path)?;
    match move_files_to_dir(&files, &keywords) {
        Ok(success_count) => {
            let msg = format!(
                "moved {} files to {} directories",
                success_count,
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
