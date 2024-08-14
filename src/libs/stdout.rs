use owo_colors::OwoColorize;

use crate::libs::errors::Error;

fn info(msg: String) {
    println!("{}", msg.blue());
}

fn warning(msg: String) {
    println!("{}", msg.bold().yellow());
}

fn success(msg: String) {
    println!("{}", msg.bold().green());
}

pub fn error(msg: String) {
    println!("{}", msg.red());
}

pub fn already_moved(filename: String) {
    warning(format!("already moved: {}", filename));
}

pub fn already_exists(filename: String) {
    warning(format!("already exists: {}", filename));
}

pub fn moved(filename: String, dst: String) {
    info(format!("moved: {} → {}", filename, dst));
}

pub fn print_result(keywords_len: usize, result: Result<Vec<String>, Error>) {
    match result {
        Ok(result) => {
            if result.len() == 0 {
                info("no files are moved.".to_string());
            } else {
                let msg = format!(
                    "moved {} files to {} directories.",
                    result.len(),
                    keywords_len
                );
                success(msg)
            }
        }
        Err(err) => {
            error(format!("{}", err));
        }
    };
}
