use crate::common::types::Solution;

fn two_sum(nums: &[i32], target: i32) -> [usize; 2] {
    let len = nums.len();
    for i in 0..len {
        if i + 1 < len {
            for j in i + 1..len {
                if nums[i] + nums[j] == target {
                    return [i, j];
                }
            }
        }
    }
    panic!("bad input");
}

/// https://leetcode.cn/problems/two-sum/
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let ret = two_sum(&nums, target);
        let mut vec = Vec::new();
        vec.push(ret[0] as i32);
        vec.push(ret[1] as i32);
        return vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn verify(nums: Vec<i32>, target: i32, expected: &Vec<i32>) {
        println!("input = {:?}", &nums);
        let ret = Solution::two_sum(nums, target);
        println!(
            "target = {},excepted = {:?}, result = {:?}",
            target, expected, ret
        );
        assert_eq!(expected, &ret);
    }
    #[test]
    fn test() {
        verify(vec![5, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9], 10, &vec![0, 1]);
        verify(vec![5, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9], 6, &vec![0, 2]);
        verify(vec![5, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9], 9, &vec![0, 5]);
        verify(vec![5, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9], 8, &vec![0, 4]);
        verify(vec![5, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9], 12, &vec![0, 8]);
        verify(vec![5, 5, 1, 2, 3, 4, 5, 6, 7, 8, 9], 6, &vec![0, 2]);
    }
}
