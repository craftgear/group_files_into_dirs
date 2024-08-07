use crate::errors::Error;

pub fn parse_args(keywords: String, path: String) -> Result<(Vec<String>, String), Error> {
    let keywords = keywords
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    keywords.iter().try_for_each(|x| {
        if x.len() < 2 {
            return Err(Error::KeywordLengthError(
                "keyword length must be more than 2".to_string(),
            ));
        }
        return Ok(());
    })?;

    if cfg!(windows) {
        if path.ends_with("\"") {
            let win_path = path.trim_end_matches("\"").to_string();
            return Ok((keywords, win_path));
        }
    }

    Ok((keywords, path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_ok() -> Result<(), Error> {
        let (keywords, path) = parse_args("aa,bb,cc".to_string(), "path".to_string())?;
        println!("{:?}, {:?}", keywords, path);

        assert_eq!(
            keywords,
            vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]
        );
        assert_eq!(path, "path".to_string());
        Ok(())
    }

    #[test]
    fn test_parse_args_keyword_is_too_short() {
        if let Err(e) = parse_args("a,b,c".to_string(), "path".to_string()) {
            assert_eq!(
                e.to_string(),
                "keyword length error: keyword length must be more than 2"
            );
        };
    }

    #[test]
    fn test_parse_args_keyword_can_parse_windows_terminal_path() {
        if let Err(e) = parse_args("a,b,c".to_string(), "path\"".to_string()) {
            assert_eq!(
                e.to_string(),
                "keyword length error: keyword length must be more than 2"
            );
        };
    }
}
