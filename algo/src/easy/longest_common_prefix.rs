use crate::common::types::Solution;

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut sort_str = strs.clone();
    sort_str.sort_by(|a, b| a.len().cmp(&b.len()));
    let first = sort_str.first().unwrap();
    let remian = &sort_str[1..];
    let mut result = String::with_capacity(first.len());
    'root: for (i, ch) in first.chars().enumerate() {
        for str in remian {
            let cmp = str.chars().nth(i);
            if cmp.is_some() && cmp.unwrap() != ch {
                break 'root;
            }
        }
        result.push(ch);
    }
    return result;
}

/// https://leetcode.cn/problems/longest-common-prefix/
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        return longest_common_prefix(strs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn verify(input: Vec<String>, expected: &str) {
        println!("input = {:?}", &input);
        let ret = Solution::longest_common_prefix(input);
        println!("excepted = {:?}, result = {:?}", expected, ret);
        assert_eq!(expected, ret);
    }
    #[test]
    fn test() {
        let strs_1 = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let strs_2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let strs_3 = vec!["".to_string(), "b".to_string()];
        verify(strs_1, "fl");
        verify(strs_2, "");
        verify(strs_3, "");
    }
}
