use crate::common::types::Solution;

struct Roman {
    roman: char,
    vaule: i32,
}

/// I             1
/// V             5
/// X             10
/// L             50
/// C             100
/// D             500
/// M             1000
///
impl Roman {
    const I: Roman = Roman {
        roman: 'I',
        vaule: 1,
    };
    const V: Roman = Roman {
        roman: 'V',
        vaule: 5,
    };
    const X: Roman = Roman {
        roman: 'X',
        vaule: 10,
    };
    const L: Roman = Roman {
        roman: 'L',
        vaule: 50,
    };
    const C: Roman = Roman {
        roman: 'C',
        vaule: 100,
    };
    const D: Roman = Roman {
        roman: 'D',
        vaule: 500,
    };
    const M: Roman = Roman {
        roman: 'M',
        vaule: 1000,
    };

    fn from(value: char) -> Roman {
        match value {
            'I' => Roman::I,
            'V' => Roman::V,
            'X' => Roman::X,
            'L' => Roman::L,
            'C' => Roman::C,
            'D' => Roman::D,
            'M' => Roman::M,
            _ => panic!("bad char : {}", value),
        }
    }

    fn merge_front(&self, front: &Roman) -> i32 {
        match front.roman {
            'I' => match self.roman {
                'V' | 'X' => -1,
                _ => 0,
            },
            'X' => match self.roman {
                'L' | 'C' => -10,
                _ => 0,
            },
            'C' => match self.roman {
                'D' | 'M' => -100,
                _ => 0,
            },
            _ => 0,
        }
    }
}

fn roman_to_int(s: &str) -> i32 {
    let mut total: i32 = 0;
    let chars: Vec<char> = s.chars().collect();
    let s = &chars[..];
    let mut front: Option<Roman> = None;
    for i in 0..s.len() {
        let current: Roman = Roman::from(s[i]);
        if front.is_some() {
            total = total + current.merge_front(&front.unwrap()) * 2;
        }
        total = total + current.vaule;
        front = Some(current);
    }
    return total;
}

/// https://leetcode.cn/problems/roman-to-integer/
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        return roman_to_int(s.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn verify(input: &str, expected: i32) {
        println!("input = {}", input);
        let ret = Solution::roman_to_int(input.to_string());
        println!("excepted = {}, result = {}", expected, ret);
        assert_eq!(expected, ret);
    }

    #[test]
    fn test() {
        verify("III", 3);
        verify("IV", 4);
        verify("IX", 9);
        verify("LVIII", 58);
        verify("MCMXCIV", 1994);
    }
}
