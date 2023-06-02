// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match (root) {
            Some(root) => {
                let right = Self::helper(root.borrow().right.clone());
                let left = Self::helper(root.borrow().left.clone());
                if right == -1 || left == -1 || (right - left).abs() > 1 {
                    return -1;
                }
                (right + 1).max(left + 1)
            }
            None => 0,
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if Solution::helper(root) == -1 {
            return false;
        }
        true
    }
}
