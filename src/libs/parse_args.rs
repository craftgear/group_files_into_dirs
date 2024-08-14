use crate::libs::errors::Error;

pub fn parse_args(keywords: String) -> Result<Vec<String>, Error> {
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

    if keywords.is_empty() {
        return Err(Error::NoKeywordsFound);
    }

    Ok(keywords)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args_ok() -> Result<(), Error> {
        let keywords = parse_args("aa,bb,cc".to_string())?;

        assert_eq!(
            keywords,
            vec!["aa".to_string(), "bb".to_string(), "cc".to_string()]
        );
        Ok(())
    }

    #[test]
    fn test_parse_args_keyword_is_too_short() {
        if let Err(e) = parse_args("a,b,c".to_string()) {
            assert_eq!(
                e.to_string(),
                "keyword length error: keyword length must be more than 2"
            );
        };
    }
}
