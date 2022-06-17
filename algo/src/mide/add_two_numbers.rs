use crate::common::types::Solution;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn to_vec(head: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut node =  head; 
    while node.is_some() {
        let current = node.as_ref().unwrap().as_ref();
        ret.push(current.val);
        node = &current.next;
    }
    return ret;
}

// [1,2,3] to [3 -> 2 -> 1]
fn to_node(values: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;
    for val in values {
        let mut node = ListNode::new(*val);
        node.next = head;
        head = Some(Box::new(node));
    }
    return head;
}

fn get_or_default(vec: &Vec<i32>,index: usize,def_val: i32) -> i32 {
    match vec.get(index){
        Some(v) => *v,
        None => def_val,
    }
}


/// https://leetcode.cn/problems/add-two-numbers/
impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let v1 = to_vec(&l1);
        let v2 = to_vec(&l2);
        if v1.is_empty(){
            return to_node(v2.as_slice());
        }
        if v2.is_empty(){
            return to_node(v1.as_slice());
        }
        let mut sum :Vec<i32> = Vec::new();
        let mut add = 0;
        for i in 0 .. v1.len().max(v2.len()) { 
            let mut val = add;
            val += get_or_default(&v1,i,0);
            val += get_or_default(&v2,i,0);
            add = val / 10;
            sum.push(val % 10);
        }
        if add > 0 {
            sum.push(add);
        }
        sum.reverse();
        return to_node(&sum);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// l1 ,l2 整数的各个位, 比如 [1,2,3] 表示123
    /// expected 预期结果,同上
    fn verify(l1: &[i32], l2: &[i32], expected: &[i32],reason:&str) {
        println!("test :{:?} + {:?} = {:?},({})",l1,l2,expected,reason);
        let n1 = to_node(l1);
        let n2 = to_node(l2);
        let ret = Solution::add_two_numbers(n1, n2);
        let mut ret = to_vec(&ret);
        // reverse for display
        ret.reverse();
        println!("output = {:?}",&ret);
        assert_eq!(&ret,&expected.to_vec());
    }
    #[test]
    fn test() {
        verify(&[9,4,2],&[9,4,6,5],&[1,0,4,0,7],"942 + 9465 = 10407");
        verify(&[3,4,2],&[4,6,5],&[8,0,7],"342 + 465 = 807");
        verify(&[0],&[0],&[0],"0 + 0 = 0");
        verify(&[9,9,9,9,9,9,9],&[9,9,9,9],&[1,0,0,0,9,9,9,8],"9999999 + 9999 = 10009998");
    }
}
