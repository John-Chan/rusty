use crate::common::types::Solution;

struct Pair{
    left: char,
    right: char
}

impl Pair {
    const BRACKET: Pair = Pair {left: '(',right:')'};
    const SQ_BRACKET: Pair = Pair {left: '[',right:']'};
    const BRACE: Pair = Pair {left: '{',right:'}'};
    
    pub fn left_of(c: char) -> Option<Pair> {
        match c {
            '(' => Some(Pair::BRACKET),
            '[' => Some(Pair::SQ_BRACKET),
            '{' => Some(Pair::BRACE),
            _ => None,
        }
    }
}

fn valid_parentheses(str: &String, matched: &mut Vec<String>) -> bool {
    let mut opens : Vec<Pair> = Vec::new();
    for (_,ch) in str.chars().enumerate() {
        let left = Pair::left_of(ch);
        if left.is_some() {
            opens.push(left.unwrap());
        } else {
            let last = opens.pop();
            if last.is_none() {
                return false;
            }
            let val = last.unwrap();
            if val.right == ch {
                matched.push(val.left.to_string());
                matched.push(val.right.to_string());
            }else {
                return false;
            }
        }
    }
    opens.is_empty()
}

/// https://leetcode.cn/problems/valid-parentheses/
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut matched: Vec<String> = Vec::new();
        valid_parentheses(&s,&mut matched)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn verify(input: &str, expected: bool) {
        println!("input = {}", input);
        let mut matched: Vec<String> = Vec::new();
        let ret = valid_parentheses(&input.to_string(),&mut matched);
        println!("matched = {:?}", matched);
        println!("excepted = {}, result = {} \r\n", expected, ret);
        assert_eq!(expected, ret);
    }

    #[test]
    fn test() {
        verify("()", true);
        verify("()[]{}", true);
        verify("(]", false);
        verify("()", true);
        verify("([)]", false);
        verify("{[]}", true);
        verify("[", false);
    }
}
