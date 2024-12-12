use regex::Regex;
use std::collections::HashMap;

static EXT_REGEX_STR: &str = r"\.([a-zA-Z0-9]+)$";
static PAREN_REGEX_STR: &str = r"\((.+?)\)|\[(.+?)\]|\{(.+?)\}";
static DELIMITERS: [char; 3] = [',', '-', '_'];
static DELIMITERS_WITH_SPACE: [char; 4] = [',', '-', '_', ' '];

fn extract_file_basename(filename: &String) -> String {
    let re = Regex::new(EXT_REGEX_STR).unwrap();
    re.replace_all(filename, "").to_string()
}

pub fn extract_keywords(filename_wo_ext: &String, space_delimiters: bool) -> Vec<String> {
    let filename_wo_ext = extract_file_basename(filename_wo_ext);
    let re = Regex::new(PAREN_REGEX_STR).unwrap();
    let mut keywords = re
        .captures_iter(&filename_wo_ext)
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
            keyword_tuple.get(1).cloned() // 0. matched str, 1. captured str
        })
        .filter(|keyword| keyword.chars().count() > 1) // keywords should have more than 1 character
        .collect::<Vec<_>>();

    // カッコを削除して、残りの文字列を取得
    let rest = re.replace_all(&filename_wo_ext, "");
    let delimiters: &[char] = if space_delimiters {
        &DELIMITERS_WITH_SPACE[..]
    } else {
        &DELIMITERS[..]
    };
    let mut rest_vec = rest
        .split(delimiters)
        .filter_map(|s| {
            let s = s.trim().to_string();
            // keywords should have more than 1 character
            if s.chars().count() > 1 {
                return Some(s);
            }
            None
        })
        .collect::<Vec<String>>();

    keywords.append(&mut rest_vec);

    keywords
}

pub fn extract_keywords_and_count_from_filenames(
    filenames: &Vec<String>,
    space_delimiters: bool,
) -> HashMap<String, usize> {
    let keyword_hash: HashMap<String, usize> =
        filenames.iter().fold(HashMap::new(), |mut acc, filename| {
            let keywords = extract_keywords(filename, space_delimiters);
            keywords.iter().for_each(|keyword| {
                acc.entry(keyword.clone())
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            });
            acc
        });

    keyword_hash
}

// sort by count first, then by keyword length
pub fn sort_by_count_and_keyword_length(
    keyword_hash: HashMap<String, usize>,
) -> Vec<(String, usize)> {
    let mut histogram: HashMap<usize, Vec<String>> =
        keyword_hash
            .clone()
            .into_iter()
            .fold(HashMap::new(), |mut acc, (keyword, count)| {
                acc.entry(count)
                    .and_modify(|vec| vec.push(keyword.clone()))
                    .or_insert(vec![keyword.clone()]);
                acc
            });

    let mut count_vec = histogram.clone().into_keys().collect::<Vec<_>>();
    count_vec.sort_by(|a, b| b.cmp(&a));

    let sorted_keyword_vec = count_vec.iter().fold(vec![], |mut acc, count| {
        let keywords = histogram.get_mut(count).unwrap();
        keywords.sort_by(|a, b| {
            if b.len() == a.len() {
                let a_lower = a.to_lowercase();
                let b_lower = b.to_lowercase();
                return a_lower.chars().nth(0).cmp(&b_lower.chars().nth(0));
            }
            return b.len().cmp(&a.len());
        });
        keywords.iter().for_each(|keyword| {
            acc.push((keyword.clone(), *count));
        });
        acc
    });

    sorted_keyword_vec
}

#[allow(dead_code)]
pub fn extract_keywords_from_camel_case(filename_wo_ext: &String) -> Vec<String> {
    let modified_string = filename_wo_ext
        .chars()
        .filter_map(|c: char| match c {
            'a'..='z' => Some(c.to_string()),
            'A'..='Z' => Some(format!(" {}", c)),
            ' ' => None,
            _ => Some(c.to_string()),
        })
        .collect::<String>();

    let keywords = modified_string
        .split(' ')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    return keywords;
}

pub fn dirname_to_keyword_regexen(dirname: &String) -> Vec<Regex> {
    extract_keywords(dirname, false) // do not use spaces as delimiters?
        .into_iter()
        .filter(|k| k.len() > 1)
        .map(|k| {
            let re = regex::Regex::new(&format!(r"[\(\[\{{\-_, ]?{k}[\)\]\}}\-_, ]")).unwrap();
            re
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords_wo_spaces() {
        let filename =
            "(000)[111](222) [333(444)] (9)(00){zzz} aaa_bbb-ccc ddd,eee.txt".to_string();
        let result = extract_keywords(&filename, false);
        assert_eq!(
            result,
            vec!["000", "111", "222", "333(444)", "00", "zzz", "aaa", "bbb", "ccc ddd", "eee"]
        );
    }

    #[test]
    fn test_extract_keywords_w_spaces() {
        let filename =
            "(000)[111](222) [333(444)] (9)(00){zzz} aaa_bbb-ccc ddd,eee.txt".to_string();
        let result = extract_keywords(&filename, true);
        assert_eq!(
            result,
            vec!["000", "111", "222", "333(444)", "00", "zzz", "aaa", "bbb", "ccc", "ddd", "eee"]
        );
    }

    #[test]
    fn test_extract_from_filenames() {
        let filenames = vec![
            "(000)[111](222) [333(444)] (9)(00){zzz} aaa_bbb-ccc ddd,eee.txt".to_string(),
            "(000)[111](222) [444(555)] (9)(00){zzz} aaa_bbb-ccc ddd,fff.txt".to_string(),
            "(000)[111](222) [555(666)] (9)(00){zzz} aaa_bbb-ccc ddd,ggg.txt".to_string(),
        ];
        let result = extract_keywords_and_count_from_filenames(&filenames, true);
        let expected = HashMap::from_iter(vec![
            ("000".to_string(), 3),
            ("111".to_string(), 3),
            ("222".to_string(), 3),
            ("zzz".to_string(), 3),
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

    #[test]
    fn test_sort_by_count_and_keyword_length() {
        // write a test for sort_by_count_and_keyword_length
        let keyword_hash = HashMap::from_iter(vec![
            ("0".to_string(), 100),
            ("1111".to_string(), 10),
            ("222".to_string(), 10),
            ("zz".to_string(), 10),
            ("ああ".to_string(), 10),
            ("aaaaa".to_string(), 3),
            ("bbbb".to_string(), 3),
            ("ccc".to_string(), 3),
            ("dd".to_string(), 3),
            ("eeeee".to_string(), 1),
            ("いい".to_string(), 1),
            ("gg".to_string(), 1),
        ]);
        let result = sort_by_count_and_keyword_length(keyword_hash);

        let expected = vec![
            ("0".to_string(), 100),
            ("ああ".to_string(), 10),
            ("1111".to_string(), 10),
            ("222".to_string(), 10),
            ("zz".to_string(), 10),
            ("aaaaa".to_string(), 3),
            ("bbbb".to_string(), 3),
            ("ccc".to_string(), 3),
            ("dd".to_string(), 3),
            ("いい".to_string(), 1),
            ("eeeee".to_string(), 1),
            ("gg".to_string(), 1),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_extract_keywords_from_camel_case() {
        let filename = "camelCase FileName Could BeParsed".to_string();
        let result = extract_keywords_from_camel_case(&filename);
        assert_eq!(
            result,
            vec!["camel", "Case", "File", "Name", "Could", "Be", "Parsed"]
        );
    }

    #[test]
    fn test_dirname_to_keyword_regexen() {
        let dirname = "inquiry_[a_company] (2024)".to_string();
        let result = dirname_to_keyword_regexen(&dirname);

        let result_str = result.iter().map(|re| re.as_str()).collect::<Vec<&str>>();

        let expected = vec![
            r"[\(\[\{\-_, ]?inquiry[\)\]\}\-_, ]",
            r"[\(\[\{\-_, ]?a_company[\)\]\}\-_, ]",
            r"[\(\[\{\-_, ]?2024[\)\]\}\-_, ]",
        ];

        assert!(result_str.iter().all(|x| expected.contains(&x)));
    }
}
