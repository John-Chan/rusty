use crate::common::types::Solution;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn collect(root: &Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) {
    if let Some(rf) = root {
        let node = rf.borrow();
        collect(&node.left, vec);
        vec.push(node.val);
        collect(&node.right, vec);
    }
}

/// https://leetcode.cn/problems/binary-tree-inorder-traversal/
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut vec = vec![];
        if let Some(r) = root {
            collect(&Some(r), &mut vec);
        }
        return vec;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn verify(input: Option<Rc<RefCell<TreeNode>>>, expected: &Vec<i32>) {
        let ret = Solution::inorder_traversal(input);
        println!("excepted = {:?}, result = {:?}", expected, &ret);
        assert_eq!(expected, &ret);
    }

    #[test]
    fn test() {
        let mut n1 = TreeNode::new(1);
        let mut n2 = TreeNode::new(2);
        let n3 = TreeNode::new(3);
        n2.left = Some(Rc::new(RefCell::new(n3)));
        n1.right = Some(Rc::new(RefCell::new(n2)));
        let root = Rc::new(RefCell::new(n1));

        verify(Some(root), &vec![1, 3, 2]);
        verify(None, &vec![]);
        verify(Some(Rc::new(RefCell::new(TreeNode::new(1)))), &vec![1]);
    }
}
