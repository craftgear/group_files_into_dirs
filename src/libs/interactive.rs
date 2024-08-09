use regex::Regex;
use std::collections::HashMap;
// use unicode_normalization::UnicodeNormalization;

// return x.as_str().nfc().collect::<String>();
fn extract_keywords(filename: &String) -> Vec<String> {
    let re = Regex::new(r"\((.+?)\)|\[(.+?)\]").unwrap();
    let mut keywords = re
        .captures_iter(filename)
        .filter_map(|captures| {
            let keyword_tuple = captures
                .iter()
                .filter_map(|capture| {
                    if let Some(x) = capture {
                        return Some(x.as_str().to_string());
                    }
                    None
                })
                .collect::<Vec<_>>();

            if keyword_tuple.len() != 2 {
                return None;
            }
            keyword_tuple.get(1).cloned()
        })
        .filter(|keyword| keyword.len() >= 2) // keywords should have more than 2 characters
        .collect::<Vec<_>>();

    // ()[]を削除して、残りの文字列を取得
    let rest = re.replace_all(filename, "");
    let ext_regex = Regex::new(r"\.([a-zA-Z0-9]+)$").unwrap();
    let mut splitted_rest = ext_regex
        .replace(&rest, "")
        .split([',', '-', '_', ' '])
        .filter_map(|s| {
            let s = s.to_string();
            if s.len() >= 2 {
                // keywords should have more than 2 characters
                return Some(s);
            }
            None
        })
        .collect::<Vec<String>>();

    keywords.append(&mut splitted_rest);

    keywords
}

fn extract_keywords_from_filenames(filenames: &Vec<String>) -> HashMap<String, usize> {
    let keyword_hash: HashMap<String, usize> =
        filenames.iter().fold(HashMap::new(), |mut acc, filename| {
            let keywords = extract_keywords(filename);
            keywords.iter().for_each(|keyword| {
                acc.entry(keyword.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });
            acc
        });

    keyword_hash
}

pub fn execute(filenames: Vec<String>) -> Vec<(String, usize)> {
    let keyword_hash = extract_keywords_from_filenames(&filenames);
    let mut keyword_vec: Vec<(String, usize)> = keyword_hash.into_iter().collect();
    keyword_vec.sort_by(|a, b| b.1.cmp(&a.1));

    // filter keywords that appear more than twice.
    let keywords = keyword_vec
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .collect::<Vec<_>>();
    keywords
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords() {
        let filename = "(000)[111](222) [333(444)] (9)(00) aaa_bbb-ccc ddd,eee.txt".to_string();
        let result = extract_keywords(&filename);
        assert_eq!(
            result,
            vec!["000", "111", "222", "333(444)", "00", "aaa", "bbb", "ccc", "ddd", "eee"]
        );
    }

    #[test]
    fn test_extract_keywords_from_filenames() {
        let filenames = vec![
            "(000)[111](222) [333(444)] (9)(00) aaa_bbb-ccc ddd,eee.txt".to_string(),
            "(000)[111](222) [444(555)] (9)(00) aaa_bbb-ccc ddd,fff.txt".to_string(),
            "(000)[111](222) [555(666)] (9)(00) aaa_bbb-ccc ddd,ggg.txt".to_string(),
        ];
        let result = extract_keywords_from_filenames(&filenames);
        let expected = HashMap::from_iter(vec![
            ("000".to_string(), 3),
            ("111".to_string(), 3),
            ("222".to_string(), 3),
            ("00".to_string(), 3),
            ("333(444)".to_string(), 1),
            ("444(555)".to_string(), 1),
            ("555(666)".to_string(), 1),
            ("aaa".to_string(), 3),
            ("bbb".to_string(), 3),
            ("ccc".to_string(), 3),
            ("ddd".to_string(), 3),
            ("eee".to_string(), 1),
            ("fff".to_string(), 1),
            ("ggg".to_string(), 1),
        ]);
        assert_eq!(result, expected);
    }
}
