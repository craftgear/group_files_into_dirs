use regex::Regex;
use std::collections::HashMap;
// use unicode_normalization::UnicodeNormalization;
//

static EXT_REGEX_STR: &str = r"\.([a-zA-Z0-9]+)$";
static PAREN_REGEX_STR: &str = r"\((.+?)\)|\[(.+?)\]|\{(.+?)\}";
static DELIMITERS: [char; 4] = [',', '-', '_', ' '];

fn extract_file_basename(filename: &String) -> String {
    let re = Regex::new(EXT_REGEX_STR).unwrap();
    re.replace_all(filename, "").to_string()
}

// return x.as_str().nfc().collect::<String>();
fn extract_keywords(filename_wo_ext: &String) -> Vec<String> {
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
        .filter(|keyword| keyword.len() > 1) // keywords should have more than 1 character, excludeing parentheses
        .collect::<Vec<_>>();

    // ()[]を削除して、残りの文字列を取得
    let rest = re.replace_all(&filename_wo_ext, "");
    let mut rest_vec = rest
        .split(DELIMITERS)
        .filter_map(|s| {
            let s = s.to_string();
            // keywords should have more than 1 character
            if s.len() > 1 {
                return Some(s);
            }
            None
        })
        .collect::<Vec<String>>();

    keywords.append(&mut rest_vec);

    keywords
}

pub fn extract_keywords_from_filenames(filenames: &Vec<String>) -> HashMap<String, usize> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_keywords() {
        let filename =
            "(000)[111](222) [333(444)] (9)(00){zzz} aaa_bbb-ccc ddd,eee.txt".to_string();
        let result = extract_keywords(&filename);
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
        let result = extract_keywords_from_filenames(&filenames);
        let expected = HashMap::from_iter(vec![
            ("000".to_string(), 3),
            ("111".to_string(), 3),
            ("222".to_string(), 3),
            ("00".to_string(), 3),
            ("zzz".to_string(), 3),
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
