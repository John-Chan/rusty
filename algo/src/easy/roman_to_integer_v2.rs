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

    fn value_with_pair(&self, right: &Roman) -> i32 {
        match self.roman {
            'I' => match right.roman {
                'V' | 'X' => -1,
                _ => self.vaule,
            },
            'X' => match right.roman {
                'L' | 'C' => -10,
                _ => self.vaule,
            },
            'C' => match right.roman {
                'D' | 'M' => -100,
                _ => self.vaule,
            },
            _ => self.vaule,
        }
    }
}

fn parse_roman(s: &str) -> i32 {
    let mut value: i32 = 0;
    let mut last: Option<Roman> = None;
    for r in s.chars().rev().map(|c| Roman::from(c)) {
        value += match last {
            Some(v) => {
                // println!("{:?} {:?} = {:?}",r.roman,v.roman,r.value_with_pair(&v));
                r.value_with_pair(&v)
            }
            _ => r.vaule,
        };
        last = Some(r);
    }

    value
}

/// https://leetcode.cn/problems/roman-to-integer/
impl Solution {
    pub fn roman_to_int_v2(s: String) -> i32 {
        return parse_roman(s.as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn verify(input: &str, expected: i32) {
        println!("input = {}", input);
        let ret = Solution::roman_to_int_v2(input.to_string());
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
