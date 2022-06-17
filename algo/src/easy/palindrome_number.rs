use crate::common::types::Solution;

fn is_alindrome(x: i32) -> bool {
    let str = x.to_string();
    let raw = str.as_bytes();
    let len = str.len();
    for i in 0..len / 2 {
        if raw[i] != raw[len - i - 1] {
            return false;
        }
    }
    return true;
}

/// https://leetcode.cn/problems/palindrome-number/
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        return is_alindrome(x);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn verify(input: i32, expected: bool) {
        println!("input = {}", input);
        let ret = Solution::is_palindrome(input);
        println!("excepted = {:?}, result = {:?}", expected, ret);
        assert_eq!(expected, ret);
    }
    #[test]
    fn test() {
        verify(121, true);
        verify(-121, false);
    }
}
