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
    pub fn helper(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        match (root) {
            Some(root) => {
                let (r, rmax) = Self::helper(root.borrow().right.clone());
                let (l, lmax) = Self::helper(root.borrow().left.clone());
                ((r + 1).max(l + 1), (r + l).max(rmax).max(lmax))
            }
            None => (0, 0),
        }
    }
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (_, d) = Solution::helper(root);
        d
    }
}
